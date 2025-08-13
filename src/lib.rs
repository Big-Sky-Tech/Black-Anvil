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

pub fn vendor_dependencies(project_path: &Path, install_dir: &Path) -> Result<()> {
    let status = Command::new("cargo")
        .arg("vendor")
        .current_dir(project_path)
        .status()
        .context("Failed to run cargo vendor")?;
    if !status.success() {
        anyhow::bail!("cargo vendor failed");
    }
    let vendor_dir = project_path.join("vendor");
    let dest = install_dir.join("vendor");
    if !vendor_dir.exists() {
        anyhow::bail!("Vendor directory does not exist after cargo vendor: {}", vendor_dir.display());
    }
    copy_dir(&vendor_dir, &dest)?;
    Ok(())
}

fn copy_dir(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)
        .with_context(|| format!("Failed to create directory {}", dst.display()))?;
    for entry in fs::read_dir(src)
        .with_context(|| format!("Failed to read directory {}", src.display()))? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest_path = dst.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir(&entry.path(), &dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}
