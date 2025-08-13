# Black Anvil

Black Anvil is an experimental Rust-based installer that compiles and installs Rust
projects. It provides a simple interactive command-line interface to choose build
options and installation path, then invokes `cargo build` and copies the resulting
binary to the selected location.

This is an early prototype focusing on compiling Rust code before installation.
Future versions aim to provide a richer graphical interface and support for other
languages and packaging formats.

## Features

- **Interactive Mode**: Simple prompts guide you through the installation process
- **Configuration Files**: Automate installations using TOML configuration files
- **Build Options**: Support for both debug and release builds
- **Dependency Vendoring**: Optional vendoring of dependencies for offline use
- **Cross-platform**: Works on Linux, macOS, and Windows

## Prerequisites

Before using Black Anvil, ensure you have:

- [Rust](https://rustup.rs/) (1.70.0 or later)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes with Rust)
- Git (for cloning repositories)

## Installation

### From Source

```bash
git clone https://github.com/Big-Sky-Tech/Black-Anvil.git
cd Black-Anvil
cargo build --release
```

The binary will be available at `target/release/black-anvil`.

## Usage

Black Anvil can be used in two modes: interactive mode and configuration file mode.

### Interactive Mode

Run Black Anvil without any arguments to enter interactive mode:

```bash
./target/release/black-anvil
```

You'll be prompted for:
- **Path to Rust project**: The directory containing the Rust project to build (default: current directory)
- **Build type**: Choose between "release" (optimized) or "debug" (faster compilation)
- **Installation directory**: Where to copy the built binary (default: ./install)
- **Vendor dependencies**: Whether to vendor dependencies for offline use (default: no)

### Configuration File Mode

Create a TOML configuration file and pass it as an argument:

```bash
./target/release/black-anvil config.toml
```

#### Configuration File Format

```toml
# config.toml
project_path = "/path/to/rust/project"
build_type = "release"  # or "debug"
install_dir = "/usr/local/bin"
vendor = false  # true to vendor dependencies
```

All fields are optional. Missing fields will trigger interactive prompts.

## Examples

### Example 1: Install a Rust CLI Tool

```bash
# Interactive mode
./target/release/black-anvil
# Follow prompts:
# Path to Rust project: /home/user/my-rust-cli
# Build type: release
# Installation directory: /usr/local/bin
# Vendor dependencies: no
```

### Example 2: Automated Installation

Create `install-config.toml`:
```toml
project_path = "./my-project"
build_type = "release"
install_dir = "./bin"
vendor = true
```

Run:
```bash
./target/release/black-anvil install-config.toml
```

### Example 3: Development Setup

Create `dev-config.toml`:
```toml
project_path = "."
build_type = "debug"
install_dir = "./dev-install"
vendor = false
```

## What Black Anvil Does

1. **Validates** the target Rust project has a valid `Cargo.toml`
2. **Builds** the project using `cargo build` (with `--release` if specified)
3. **Copies** the resulting binary to the specified installation directory
4. **Vendors** dependencies (if requested) using `cargo vendor`

## Troubleshooting

### Common Issues

**Error: "Failed to run cargo build"**
- Ensure the target directory contains a valid Rust project with `Cargo.toml`
- Check that all dependencies can be resolved
- Verify Rust and Cargo are properly installed

**Error: "cargo build failed"**
- Check the project compiles with `cargo build` directly
- Review any compilation errors in the project
- Ensure all required system dependencies are installed

**Error: "Failed to copy binary"**
- Verify the installation directory is writable
- Check available disk space
- Ensure the binary was successfully built in the target directory

**Error: "cargo vendor failed"**
- Ensure the project has dependencies to vendor
- Check network connectivity for downloading dependencies
- Verify sufficient disk space for vendored dependencies

### Debug Mode

Run Black Anvil with a debug build for more verbose error messages:

```bash
cargo run -- [config.toml]
```

## Contributing

Contributions are welcome! This is an experimental project with room for improvement.

### Development Setup

1. Clone the repository
2. Run `cargo build` to build the project
3. Run `cargo test` to run tests
4. Run `cargo run` for development testing

### Future Enhancements

- Command-line argument parsing with `--help`
- Support for multiple binary targets
- GUI interface
- Package manager integration
- Support for other languages and build systems

## License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for details.
