use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

#[derive(Debug, PartialEq)]
enum Language {
    Rust,
    Go,
}

struct SolutionInfo {
    path: PathBuf,
    name: String,
    language: Language,
}

const RUST_SOURCE_FILES: &[&str] = &["src/lib.rs", "Cargo.toml"];
const GO_SOURCE_FILES: &[&str] = &["solution.go", "wasm.go", "go.mod"];

fn main() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("must be inside /auto-gen")
        .to_path_buf();

    let wasm_output_dir = project_root.join("frontend/wasm");
    if !wasm_output_dir.exists() {
        fs::create_dir_all(&wasm_output_dir).expect("Failed to create WASM output directory");
    }

    let solution_paths = scan_solutions_and_generate_manifest(&project_root);

    build_solution_crates(&solution_paths, &wasm_output_dir);

    println!("\n🎉 Successfully completed incremental WASM build.");
}

fn scan_solutions_and_generate_manifest(project_root: &Path) -> Vec<SolutionInfo> {
    let mut rs_members = vec![];
    let mut solution_paths = vec![];

    for year in 2023..=2024 {
        for day in 1..=25 {
            for part in 1..=2 {
                let rel_path = format!("{}/day_{:02}/task_{}", year, day, part);
                let path = project_root.join(rel_path.clone());
                let name = format!("solution_{}_{:02}_{}", year, day, part);

                let language = if path.join("src/lib.rs").exists() {
                    rs_members.push(format!("  \"{}\",", rel_path));
                    Language::Rust
                } else if path.join("wasm.go").exists() {
                    Language::Go
                } else {
                    continue;
                };

                solution_paths.push(SolutionInfo {
                    path,
                    name,
                    language,
                });
            }
        }
    }

    // Dynamic root Cargo.toml generation (for Rust workspace)
    let root_cargo = project_root.join("Cargo.toml");
    {
        let mut root_file = BufWriter::new(File::create(root_cargo).unwrap());
        writeln!(root_file, "[workspace]\nmembers = [").unwrap();
        for member in &rs_members {
            writeln!(root_file, "{}", member).unwrap();
        }
        writeln!(root_file, "  \"aoc_utils\",\n  \"auto-gen\"\n]").unwrap();
        writeln!(root_file, "resolver = \"3\"").unwrap();

        writeln!(root_file, "\n[profile.release]").unwrap();
        writeln!(root_file, "opt-level = \"z\"").unwrap(); // Optimize for size
        writeln!(root_file, "lto = true").unwrap(); // Link-Time Optimization
        writeln!(root_file, "codegen-units = 1").unwrap(); // Best LTO results
        writeln!(root_file, "panic = \"abort\"").unwrap(); // Minimize code size

        root_file.flush().unwrap();
    }

    println!(
        "✅ Generated dynamic root Cargo.toml with {} Rust members.",
        rs_members.len()
    );

    solution_paths
}

fn get_output_path(solution_info: &SolutionInfo, wasm_output_dir: &Path) -> PathBuf {
    match solution_info.language {
        Language::Rust => wasm_output_dir.join(format!("{}_bg.wasm", solution_info.name)),
        Language::Go => wasm_output_dir.join(format!("{}.wasm", solution_info.name)),
    }
}

const fn get_source_files(language: &Language) -> &'static [&'static str] {
    match language {
        Language::Rust => RUST_SOURCE_FILES,
        Language::Go => GO_SOURCE_FILES,
    }
}

fn is_recompile_needed(solution_info: &SolutionInfo, wasm_output_dir: &Path) -> bool {
    let output_path = get_output_path(solution_info, wasm_output_dir);
    let solution_path = &solution_info.path;
    let name = &solution_info.name;

    // Case 1: No WASM file currently exists
    if !output_path.exists() {
        println!("  - ➡️ Compiling {} (WASM file not found).", name);
        return true;
    }

    // Case 2: Source file is newer than WASM file
    let wasm_mtime = fs::metadata(&output_path)
        .and_then(|m| m.modified())
        .expect("WASM file has no modification time");

    let source_files = get_source_files(&solution_info.language);

    for file_name in source_files {
        let source_path = solution_path.join(file_name);

        if !source_path.exists() {
            continue;
        }

        let source_mtime = fs::metadata(&source_path)
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        if source_mtime > wasm_mtime {
            println!("  - ➡️ Recompiling {} ({} modified).", name, file_name);
            return true;
        }
    }

    // Case 3: No recompile needed
    false
}

fn get_build_commands(solution_info: &SolutionInfo, wasm_output_dir: &Path) -> Vec<Command> {
    match solution_info.language {
        Language::Rust => {
            let mut wasm_pack_cmd = Command::new("wasm-pack");
            wasm_pack_cmd
                .arg("build")
                .arg(&solution_info.path)
                .arg("--target")
                .arg("web")
                .arg("--out-dir")
                .arg(wasm_output_dir)
                .arg("--out-name")
                .arg(&solution_info.name);

            vec![wasm_pack_cmd]
        }
        Language::Go => {
            let mut go_cmd = Command::new("go");
            go_cmd
                .current_dir(&solution_info.path)
                .env("GOOS", "js")
                .env("GOARCH", "wasm")
                .arg("build")
                .arg("-o")
                .arg(wasm_output_dir.join(format!("{}_bg.wasm", solution_info.name)));

            let mut js_wrapper_cmd = Command::new("sh");
            js_wrapper_cmd.arg("-c").arg(format!(
                "sed 's&__WASM_PATH__&wasm/{name}_bg.wasm&g' {template} > {outdir}/{name}.js",
                name = solution_info.name,
                template = format!("{}/go_template.js", env!("CARGO_MANIFEST_DIR")),
                outdir = wasm_output_dir.display(),
            ));

            let mut js_glue_cmd = Command::new("sh");
            js_glue_cmd.arg("-c").arg(format!(
                "cp \"$(go env GOROOT)/lib/wasm/wasm_exec.js\" \"{}\"",
                wasm_output_dir.join("wasm_exec.js").display()
            ));

            vec![go_cmd, js_wrapper_cmd, js_glue_cmd]
        }
    }
}

fn build_solution_crates(solution_paths: &[SolutionInfo], wasm_output_dir: &Path) {
    println!("\n📦 Starting WASM compilation (Incremental)...");

    for info in solution_paths {
        if is_recompile_needed(info, wasm_output_dir) {
            let mut commands = get_build_commands(info, wasm_output_dir);

            for command in &mut commands {
                let output = command
                    .output()
                    .expect(&format!("Failed to execute builder for {}", info.name));

                if !output.status.success() {
                    eprintln!(
                        "❌ Build failed for {} ({}):",
                        info.name,
                        match info.language {
                            Language::Rust => "Rust",
                            Language::Go => "Go",
                        }
                    );
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    panic!("Build failed.");
                }
            }

            println!(
                "  - ✅ Successfully built {} ({}).",
                info.name,
                match info.language {
                    Language::Rust => "Rust",
                    Language::Go => "Go",
                }
            );
        }
    }
}
