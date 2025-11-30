use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    process::Command,
    time::SystemTime,
};

struct SolutionInfo {
    path: PathBuf,
    crate_name: String,
}

fn main() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("must be inside /auto-gen")
        .to_path_buf();
    let wasm_output_dir = project_root.join("frontend/wasm");
    let solution_paths = scan_solutions_and_generate_manifest(&project_root);

    build_solution_crates(&solution_paths, &wasm_output_dir);

    println!("\n🎉 Successfully completed incremental WASM build.");
}

fn scan_solutions_and_generate_manifest(project_root: &Path) -> Vec<SolutionInfo> {
    let mut members = vec![];
    let mut solution_paths = vec![];

    for year in 2023..=2024 {
        for day in 1..=25 {
            for part in 1..=2 {
                let path = project_root.join(format!("{}/day_{:02}/task_{}", year, day, part));

                if path.join("src/lib.rs").exists() {
                    let rel_path = path.strip_prefix(project_root).unwrap();
                    let crate_name = format!("solution_{}_{:02}_{}", year, day, part);

                    members.push(format!("  \"{}\",", rel_path.display()));
                    solution_paths.push(SolutionInfo { path, crate_name });
                }
            }
        }
    }

    let root_cargo = project_root.join("Cargo.toml");
    {
        let mut root_file = BufWriter::new(File::create(root_cargo).unwrap());

        writeln!(root_file, "[workspace]\nmembers = [").unwrap();
        for member in &members {
            writeln!(root_file, "{}", member).unwrap();
        }
        writeln!(root_file, "  \"frontend\",\n  \"auto-gen\"\n]").unwrap();

        root_file.flush().unwrap();
    }

    println!(
        "✅ Generated dynamic root Cargo.toml with {} members.",
        solution_paths.len()
    );

    solution_paths
}

fn is_recompile_needed(solution_info: &SolutionInfo, wasm_output_dir: &Path) -> bool {
    let wasm_path = wasm_output_dir.join(format!("{}_bg.wasm", solution_info.crate_name));
    let solution_path = &solution_info.path;

    // Case 1: No WASM file currently exists
    if !wasm_path.exists() {
        println!(
            "   - ➡️ Compiling {} (WASM file not found).",
            solution_info.crate_name
        );
        return true;
    }

    // Case 2: Source file is newer than WASM file
    let wasm_metadata = fs::metadata(&wasm_path).expect("Failed to get WASM metadata");
    let wasm_mtime = wasm_metadata
        .modified()
        .expect("WASM file has no modification time");

    let source_files = ["src/lib.rs", "Cargo.toml"];
    for file_name in source_files {
        let source_path = solution_path.join(file_name);
        if !source_path.exists() {
            continue;
        }

        let source_mtime = fs::metadata(&source_path)
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH);

        if source_mtime > wasm_mtime {
            println!(
                "   - ➡️ Recompiling {} ({} modified).",
                solution_info.crate_name, file_name
            );
            return true;
        }
    }

    // Case 3: No recompile needed
    false
}

fn build_solution_crates(solution_paths: &[SolutionInfo], wasm_output_dir: &Path) {
    println!("\n📦 Starting WASM compilation (Incremental)...");

    for info in solution_paths {
        if is_recompile_needed(info, wasm_output_dir) {
            let output = Command::new("wasm-pack")
                .arg("build")
                .arg(&info.path)
                .arg("--target")
                .arg("web")
                .arg("--out-dir")
                .arg(wasm_output_dir)
                .arg("--out-name")
                .arg(&info.crate_name)
                .output()
                .expect(&format!(
                    "Failed to execute wasm-pack for {}",
                    info.crate_name
                ));

            if !output.status.success() {
                eprintln!("❌ WASM build failed for {}:", info.crate_name);
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                panic!("WASM build failed.");
            } else {
                println!("   - ✅ Successfully built {}.", info.crate_name);
            }
        }
    }
}
