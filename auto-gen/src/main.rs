use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

fn main() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("must be inside /auto-gen")
        .to_path_buf();

    let frontend_dir = project_root.join("frontend");

    let mut members = vec![];
    let mut deps = vec![];
    let mut registry = vec![];

    for year in 2024..=2024 {
        for day in 1..=25 {
            for part in 1..=2 {
                let path = project_root.join(format!("{}/day_{:02}/task_{}", year, day, part));

                if path.join("src/lib.rs").exists() {
                    let rel_path = path.strip_prefix(&project_root).unwrap();
                    let crate_name = format!("solution_{}_{:02}_{}", year, day, part);

                    members.push(format!("  \"{}\",", rel_path.display()));
                    deps.push(format!(
                        "{} = {{ path = \"../{}\" }}",
                        crate_name,
                        rel_path.display()
                    ));
                    registry.push((year, day, part, crate_name));
                }
            }
        }
    }

    // === Write workspace Cargo.toml ===
    let root_cargo = project_root.join("Cargo.toml");
    let mut root_file = BufWriter::new(File::create(root_cargo).unwrap());
    writeln!(root_file, "[workspace]\nmembers = [").unwrap();
    for member in &members {
        writeln!(root_file, "{}", member).unwrap();
    }
    writeln!(root_file, "  \"frontend\",\n  \"auto-gen\"\n]").unwrap();

    // === Patch frontend/Cargo.toml ===
    let frontend_cargo = frontend_dir.join("Cargo.toml");
    let original = fs::read_to_string(&frontend_cargo).unwrap_or_default();
    let mut new_frontend = String::new();

    for line in original.lines() {
        if line.trim() == "# (This will break the auto-generation)" {
            break;
        }
        new_frontend.push_str(line);
        new_frontend.push('\n');
    }
    new_frontend.push_str("# (This will break the auto-generation)\n\n");

    for dep in &deps {
        new_frontend.push_str(dep);
        new_frontend.push('\n');
    }

    fs::write(&frontend_cargo, new_frontend).unwrap();

    // === Generate frontend/src/lib.rs ===
    let generated = frontend_dir.join("src/lib.rs");
    let mut file = BufWriter::new(File::create(&generated).unwrap());

    for (_, _, _, crate_name) in &registry {
        writeln!(file, "use {};", crate_name).unwrap();
    }

    writeln!(
        file,
        r#"
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(year: u16, day: u16, part: u16, input: &str) -> String {{
    match (year, day, part) {{"#
    )
    .unwrap();

    for (year, day, part, crate_name) in &registry {
        writeln!(
            file,
            "        ({}, {}, {}) => {}::solve(input).to_string(),",
            year, day, part, crate_name
        )
        .unwrap();
    }

    writeln!(
        file,
        r#"        _ => "Not implemented yet".to_string(),
    }}
}}"#
    )
    .unwrap();

    println!("✅ Generated workspace and frontend files.");
}
