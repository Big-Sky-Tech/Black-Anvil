use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct CargoPackage {
    name: String,
}

#[derive(Deserialize)]
struct CargoToml {
    package: CargoPackage,
}

pub fn build_project(project_path: &Path, build_type: &str) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.arg("build");
    if build_type == "release" {
        cmd.arg("--release");
    }
    let status = cmd
        .current_dir(project_path)
        .status()
        .context("Failed to run cargo build")?;
    if !status.success() {
        anyhow::bail!("cargo build failed");
    }
    Ok(())
}

pub fn copy_binary(project_path: &Path, build_type: &str, install_dir: &Path) -> Result<PathBuf> {
    let cargo_toml_path = project_path.join("Cargo.toml");
    let cargo_toml_str = fs::read_to_string(&cargo_toml_path)
        .with_context(|| format!("Failed to read {}", cargo_toml_path.display()))?;
    let cargo_toml: CargoToml =
        toml::from_str(&cargo_toml_str).context("Failed to parse Cargo.toml")?;
    let package_name = cargo_toml.package.name;

    let binary_name = if cfg!(windows) {
        format!("{}.exe", package_name)
    } else {
        package_name
    };

    let source = project_path
        .join("target")
        .join(build_type)
        .join(&binary_name);
    fs::create_dir_all(install_dir)
        .with_context(|| format!("Failed to create directory {}", install_dir.display()))?;
    let dest = install_dir.join(&binary_name);
    fs::copy(&source, &dest).with_context(|| {
        format!(
            "Failed to copy binary from {} to {}",
            source.display(),
            dest.display()
        )
    })?;

    Ok(dest)
}
