use std::env;
use std::path::Path;

use anyhow::Result;
use dialoguer::{Confirm, Input, Select};

use black_anvil::{build_project, copy_binary, vendor_dependencies};

mod config;
use config::Config;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = if let Some(path) = args.get(1) {
        Config::from_file(Path::new(path))?
    } else {
        Config::default()
    };

    let project_path = if let Some(p) = config.project_path.clone() {
        p
    } else {
        Input::new()
            .with_prompt("Path to Rust project")
            .default(".".into())
            .interact_text()?
    };

    let build_type = if let Some(b) = config.build_type.clone() {
        b
    } else {
        let build_types = ["release", "debug"];
        let selection = Select::new()
            .with_prompt("Build type")
            .items(&build_types)
            .default(0)
            .interact()?;
        build_types[selection].into()
    };

    let install_dir = if let Some(i) = config.install_dir.clone() {
        i
    } else {
        Input::new()
            .with_prompt("Installation directory")
            .default("./install".into())
            .interact_text()?
    };

    let vendor = if let Some(v) = config.vendor {
        v
    } else {
        Confirm::new()
            .with_prompt("Vendor dependencies?")
            .default(false)
            .interact()?
    };

    build_project(Path::new(&project_path), &build_type)?;
    let dest = copy_binary(
        Path::new(&project_path),
        &build_type,
        Path::new(&install_dir),
    )?;

    if vendor {
        vendor_dependencies(Path::new(&project_path), Path::new(&install_dir))?;
    }

    println!(
        "Installed {:?} to {}",
        dest.file_name().unwrap(),
        dest.display()
    );

    Ok(())
}
