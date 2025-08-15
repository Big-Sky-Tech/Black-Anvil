# Black Anvil

**A powerful, extensible Rust project builder and installer with Starlark scripting and modern UI**

Black Anvil is an advanced Rust-based build and installation tool that combines the flexibility of command-line automation with the power of embedded scripting and modern graphical interfaces. It provides multiple ways to build, configure, and install Rust projects with support for complex deployment scenarios.

## 🚀 Key Features

### Multi-Interface Support
- **Command-Line Interface**: Interactive prompts and automated configuration
- **Tauri Frontend**: Modern web-based GUI for visual project management
- **Configuration Files**: TOML-based automation for CI/CD and batch operations

### Advanced Build Capabilities
- **Debug & Release Builds**: Choose optimal build profiles for your use case
- **Dependency Vendoring**: Offline installations with all dependencies included
- **Cross-Platform Support**: Works on Windows, macOS, and Linux
- **Flexible Installation**: User-local, system-wide, or custom directory installations

### Starlark Scripting Integration
- **Embedded Scripting**: Customize build and installation logic with Starlark
- **Extensible Configuration**: Beyond simple TOML with programmable workflows
- **Automation Support**: Perfect for complex CI/CD pipelines

## 📦 Installation

### Prerequisites

- **Rust toolchain** (1.70 or later)
- **Cargo** (included with Rust)
- **Node.js and npm** (for Tauri frontend development)

### Building from Source

```bash
# Clone the repository
git clone https://github.com/Big-Sky-Tech/Black-Anvil.git
cd Black-Anvil

# Build the CLI tool
cargo build --release

# The binary will be available at ./target/release/black-anvil
```

### Self-installer

Build and install Black Anvil using the project itself:

```bash
./scripts/self_installer.sh /usr/local/bin
```

The argument specifies the installation directory and defaults to `/usr/local/bin`.

### Building the Tauri Frontend

```bash
# Navigate to the Tauri app directory
cd tauri-app

# Install dependencies
npm install

# Build the Tauri application
npm run tauri build
```

## 🎯 Quick Start

### Interactive Mode

Run Black Anvil without arguments to enter interactive mode:

```bash
./target/release/black-anvil
```

You'll be prompted for:
- **Project path**: Directory containing your Rust project
- **Build type**: Choose between `debug` or `release`
- **Installation directory**: Where to install the compiled binary
- **Vendor dependencies**: Whether to include offline dependencies

### Configuration File Mode

Create a `config.toml` file:

```toml
project_path = "/path/to/your/rust/project"
build_type = "release"
install_dir = "/usr/local/bin"
vendor = false
```

Run with the configuration:

```bash
./target/release/black-anvil config.toml
```

### Tauri GUI Mode

Launch the graphical interface:

```bash
cd tauri-app
npm run tauri dev
```

The GUI provides:
- Visual project selection
- Build configuration options
- Installation progress tracking
- Advanced settings management

## 📖 Detailed Documentation

- **[USAGE.md](USAGE.md)** - Step-by-step usage guide with examples
- **[CONFIG.md](CONFIG.md)** - Complete configuration reference
- **[examples/](examples/)** - Working configuration examples
- **[STARLARK.md](STARLARK.md)** - Starlark scripting guide

## 🔧 Configuration Options

| Option | Type | Description | Default |
|--------|------|-------------|---------|
| `project_path` | String | Path to Rust project directory | Current directory |
| `build_type` | String | Build profile (`debug` or `release`) | Interactive prompt |
| `install_dir` | String | Installation destination | `./install` |
| `vendor` | Boolean | Include vendored dependencies | `false` |

## 🌟 Example Use Cases

### Development Workflow
- Quick debug builds for testing
- User-local installations for development tools
- Automated builds with dependency caching

### Production Deployment
- Release builds with optimizations
- System-wide installations for servers
- Offline deployments with vendored dependencies

### CI/CD Integration
- Automated builds in containerized environments
- Reproducible builds with locked dependencies
- Multi-platform deployment automation

## 🔧 Troubleshooting

### Common Issues

**Build fails with "cargo not found"**
- Ensure Rust and Cargo are installed and in your PATH
- Try `rustup update` to update your Rust installation

**Permission denied during installation**
- Use `sudo` for system-wide installations
- Choose user-local directories like `~/.local/bin`
- Verify target directory permissions

**Missing dependencies in offline mode**
- Run `cargo vendor` in your project first
- Ensure all dependencies are properly cached
- Check that `vendor` option is enabled

## 🤝 Contributing

Black Anvil is an open-source project welcoming contributions:

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Make your changes** with tests and documentation
4. **Commit your changes** (`git commit -m 'Add amazing feature'`)
5. **Push to the branch** (`git push origin feature/amazing-feature`)
6. **Open a Pull Request**

### Development Setup

```bash
# Clone your fork
git clone https://github.com/your-username/Black-Anvil.git
cd Black-Anvil

# Build and test
cargo build
cargo test

# For frontend development
cd tauri-app
npm install
npm run dev
```

## 📄 License

This project is licensed under the [LICENSE](LICENSE) file in the repository.

## 🔗 Links

- **Documentation**: [docs/](docs/)
- **Examples**: [examples/](examples/)
- **Issues**: [GitHub Issues](https://github.com/Big-Sky-Tech/Black-Anvil/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Big-Sky-Tech/Black-Anvil/discussions)

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
