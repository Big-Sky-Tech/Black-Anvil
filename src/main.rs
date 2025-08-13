use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};
use dialoguer::{Input, Select};
use serde::Deserialize;

#[derive(Deserialize)]
struct CargoPackage {
    name: String,
}

#[derive(Deserialize)]
struct CargoToml {
    package: CargoPackage,
}

fn main() -> Result<()> {
    // Ask for the path to the Rust project
    let project_path: String = Input::new()
        .with_prompt("Path to Rust project")
        .default(".".into())
        .interact_text()?;

    // Choose build type
    let build_types = ["release", "debug"];
    let selection = Select::new()
        .with_prompt("Build type")
        .items(&build_types)
        .default(0)
        .interact()?;
    let build_type = build_types[selection];

    // Installation directory
    let install_dir: String = Input::new()
        .with_prompt("Installation directory")
        .default("./install".into())
        .interact_text()?;

    // Parse Cargo.toml to get package name
    let cargo_toml_path = Path::new(&project_path).join("Cargo.toml");
    let cargo_toml_str = fs::read_to_string(&cargo_toml_path)
        .with_context(|| format!("Failed to read {}", cargo_toml_path.display()))?;
    let cargo_toml: CargoToml =
        toml::from_str(&cargo_toml_str).context("Failed to parse Cargo.toml")?;
    let package_name = cargo_toml.package.name;

    // Run cargo build
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    if build_type == "release" {
        cmd.arg("--release");
    }
    let status = cmd
        .current_dir(&project_path)
        .status()
        .context("Failed to run cargo build")?;
    if !status.success() {
        anyhow::bail!("cargo build failed");
    }

    // Copy compiled binary to installation directory
    let binary_name = if cfg!(windows) {
        format!("{}.exe", package_name)
    } else {
        package_name
    };

    let source = Path::new(&project_path)
        .join("target")
        .join(build_type)
        .join(&binary_name);
    let dest_dir = Path::new(&install_dir);
    fs::create_dir_all(&dest_dir)
        .with_context(|| format!("Failed to create directory {}", dest_dir.display()))?;
    let dest = dest_dir.join(&binary_name);
    fs::copy(&source, &dest).with_context(|| {
        format!(
            "Failed to copy binary from {} to {}",
            source.display(),
            dest.display()
        )
    })?;

    println!("Installed {} to {}", binary_name, dest.display());

    Ok(())
}
