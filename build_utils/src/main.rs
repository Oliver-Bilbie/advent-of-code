use std::{
    fmt, fs,
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

#[derive(Debug, PartialEq)]
enum Language {
    Rust,
    Go,
    Python,
    Missing,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Rust => "Rust 🦀",
                Language::Go => "Go 🐹",
                Language::Python => "Python 🐍",
                Language::Missing => "Not implemented",
            }
        )
    }
}

impl Language {
    const fn src_files(&self) -> &'static [&'static str] {
        match self {
            Language::Rust => &["src/lib.rs", "Cargo.toml"],
            Language::Go => &["solution.go", "wasm.go", "go.mod"],
            Language::Python => &["solution.py"],
            Language::Missing => &[],
        }
    }
}

struct SolutionInfo {
    path: PathBuf,
    name: String,
    language: Language,
}

fn main() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("must be inside /build_utils")
        .to_path_buf();

    let wasm_output_dir = project_root.join("frontend/wasm");
    if !wasm_output_dir.exists() {
        fs::create_dir_all(&wasm_output_dir).expect("Failed to create WASM output directory");
    }

    let solution_paths = scan_solutions_and_generate_manifest(&project_root);

    build_solution_crates(&solution_paths, &wasm_output_dir);

    println!("\n🎉 Successfully completed WASM build.");
}

fn scan_solutions_and_generate_manifest(project_root: &Path) -> Vec<SolutionInfo> {
    let mut solution_paths = vec![];

    for year in 2015..=2025 {
        for day in 1..=25 {
            for part in 1..=2 {
                let rel_path = format!("{}/day_{:02}/task_{}", year, day, part);
                let path = project_root.join(rel_path.clone());
                let name = format!("solution_{}_{:02}_{}", year, day, part);

                let language = if path.join("src/lib.rs").exists() {
                    Language::Rust
                } else if path.join("wasm.go").exists() {
                    Language::Go
                } else if path.join("solution.py").exists() {
                    Language::Python
                } else {
                    Language::Missing
                };

                solution_paths.push(SolutionInfo {
                    path,
                    name,
                    language,
                });
            }
        }
    }

    solution_paths
}

fn is_recompile_needed(solution_info: &SolutionInfo, wasm_output_dir: &Path) -> bool {
    let wasm_path = wasm_output_dir.join(format!("{}_bg.wasm", solution_info.name));
    let js_path = wasm_output_dir.join(format!("{}.js", solution_info.name));
    let solution_path = &solution_info.path;

    // Case 0: No solution exists, and a js handler has already been created
    if solution_info.language == Language::Missing && js_path.exists() {
        return false;
    }

    // Case 1: No WASM file currently exists
    if !wasm_path.exists() || !js_path.exists() {
        return true;
    }

    // Case 2: Source file is newer than WASM file
    let wasm_mtime = match fs::metadata(&wasm_path).and_then(|m| m.modified()) {
        Ok(val) => val,
        Err(_) => {
            // WASM file has no modified time data so let's recompile it to be safe
            return true;
        }
    };

    for file_name in solution_info.language.src_files() {
        let source_path = solution_path.join(file_name);

        if !source_path.exists() {
            continue;
        }

        let source_mtime = fs::metadata(&source_path)
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        if source_mtime > wasm_mtime {
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

            let mut append_lang_cmd = Command::new("sh");
            append_lang_cmd.arg("-c").arg(format!(
                "printf '{}' >> {}/{}.js",
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/rust_template.js")),
                wasm_output_dir.to_string_lossy(),
                solution_info.name
            ));

            vec![wasm_pack_cmd, append_lang_cmd]
        }

        Language::Go => {
            let mut go_cmd = Command::new("tinygo");
            go_cmd
                .current_dir(&solution_info.path)
                .arg("build")
                .arg("-o")
                .arg(wasm_output_dir.join(format!("{}_bg.wasm", solution_info.name)))
                .arg("-target")
                .arg("wasm")
                .arg("-no-debug")
                .arg("-ldflags")
                .arg(format!("-X main.SolverName={}", solution_info.name))
                .arg(".");

            let mut js_wrapper_cmd = Command::new("sh");
            js_wrapper_cmd.arg("-c").arg(format!(
                "sed \
                    -e 's&__WASM_PATH__&wasm/{name}_bg.wasm&g' \
                    -e 's&__SOLVER_NAME__&{name}&g' \
                    {template} > {outdir}/{name}.js",
                name = solution_info.name,
                template = format!("{}/go_template.js", env!("CARGO_MANIFEST_DIR")),
                outdir = wasm_output_dir.display(),
            ));

            let mut js_glue_cmd = Command::new("sh");
            js_glue_cmd.arg("-c").arg(format!(
                "cp \"$(tinygo env TINYGOROOT)/targets/wasm_exec.js\" \"{}\"",
                wasm_output_dir.join("wasm_exec.js").display()
            ));

            vec![go_cmd, js_wrapper_cmd, js_glue_cmd]
        }

        Language::Python => {
            let mut cp_script_cmd = Command::new("cp");
            cp_script_cmd
                .arg(&solution_info.path.join("solution.py"))
                .arg(format!(
                    "{}/{}.py",
                    wasm_output_dir.display(),
                    solution_info.name
                ));

            let mut js_wrapper_cmd = Command::new("sh");
            js_wrapper_cmd.arg("-c").arg(format!(
                "sed \
                    -e 's&__SCRIPT_PATH__&wasm/{name}.py&g' \
                    {template} > {outdir}/{name}.js",
                name = solution_info.name,
                template = format!("{}/python_template.js", env!("CARGO_MANIFEST_DIR")),
                outdir = wasm_output_dir.display(),
            ));

            vec![cp_script_cmd, js_wrapper_cmd]
        }

        Language::Missing => {
            let mut js_handler_cmd = Command::new("cp");
            js_handler_cmd
                .arg("-n")
                .arg(format!(
                    "{}/missing_template.js",
                    env!("CARGO_MANIFEST_DIR")
                ))
                .arg(format!(
                    "{}/{}.js",
                    wasm_output_dir.display(),
                    solution_info.name
                ));

            vec![js_handler_cmd]
        }
    }
}

fn build_solution_crates(solution_paths: &[SolutionInfo], wasm_output_dir: &Path) {
    println!("\n📦 Starting WASM compilation...");

    for info in solution_paths {
        if is_recompile_needed(info, wasm_output_dir) {
            println!("  - ➡️ Compiling {}", info.name);

            let mut commands = get_build_commands(info, wasm_output_dir);

            for command in &mut commands {
                let output = command
                    .output()
                    .expect(&format!("Failed to execute builder for {}", info.name));

                if !output.status.success() {
                    eprintln!("❌ Build failed for {} ({}):", info.name, info.language,);
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                    panic!("Build failed.");
                }
            }

            println!(
                "  - ✅ Successfully built {} ({}).",
                info.name, info.language
            );
        }
    }
}
