use std::path::Path;

use anyhow::Result;
use dialoguer::{Input, Select};

use black_anvil::{build_project, copy_binary};

fn main() -> Result<()> {
    let project_path: String = Input::new()
        .with_prompt("Path to Rust project")
        .default(".".into())
        .interact_text()?;

    let build_types = ["release", "debug"];
    let selection = Select::new()
        .with_prompt("Build type")
        .items(&build_types)
        .default(0)
        .interact()?;
    let build_type = build_types[selection];

    let install_dir: String = Input::new()
        .with_prompt("Installation directory")
        .default("./install".into())
        .interact_text()?;

    build_project(Path::new(&project_path), build_type)?;
    let dest = copy_binary(
        Path::new(&project_path),
        build_type,
        Path::new(&install_dir),
    )?;

    println!(
        "Installed {:?} to {}",
        dest.file_name().unwrap(),
        dest.display()
    );

    Ok(())
}
