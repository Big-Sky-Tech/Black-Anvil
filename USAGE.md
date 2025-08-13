# Black Anvil Usage Guide

This guide provides detailed instructions for using Black Anvil to compile and install Rust projects.

## Table of Contents

- [Basic Usage](#basic-usage)
- [Interactive Mode](#interactive-mode)
- [Configuration File Mode](#configuration-file-mode)
- [Configuration Options](#configuration-options)
- [Advanced Usage](#advanced-usage)
- [Best Practices](#best-practices)

## Basic Usage

Black Anvil provides two ways to operate:

1. **Interactive Mode**: Run without arguments, answer prompts
2. **Configuration File Mode**: Provide a TOML configuration file

### Quick Start

```bash
# Interactive mode
./black-anvil

# Configuration file mode
./black-anvil my-config.toml
```

## Interactive Mode

When run without arguments, Black Anvil will prompt you for each required setting:

### Step-by-Step Walkthrough

1. **Project Path Prompt**
   ```
   Path to Rust project [.]:
   ```
   - Enter the path to your Rust project directory
   - Must contain a valid `Cargo.toml` file
   - Default: current directory (`.`)
   - Examples: `./my-project`, `/home/user/rust-cli`, `../another-project`

2. **Build Type Selection**
   ```
   Build type
   > release
     debug
   ```
   - Use arrow keys to select, press Enter to confirm
   - **release**: Optimized build, slower compilation, faster execution
   - **debug**: Fast compilation, larger binary, includes debug symbols

3. **Installation Directory**
   ```
   Installation directory [./install]:
   ```
   - Where the compiled binary will be copied
   - Directory will be created if it doesn't exist
   - Default: `./install` in current directory
   - Examples: `/usr/local/bin`, `~/.local/bin`, `./my-app/bin`

4. **Vendor Dependencies**
   ```
   Vendor dependencies? [y/N]:
   ```
   - Choose whether to vendor (download and store locally) dependencies
   - Default: No (N)
   - Useful for offline installations or reproducible builds

### Example Interactive Session

```
$ ./black-anvil
Path to Rust project [.]: ./my-rust-cli
Build type
> release
  debug
Installation directory [./install]: /usr/local/bin
Vendor dependencies? [y/N]: n

Building project...
Installed "my-rust-cli" to /usr/local/bin/my-rust-cli
```

## Configuration File Mode

Configuration files allow you to automate and script Black Anvil installations.

### Basic Configuration File

Create a file with a `.toml` extension:

```toml
# basic-config.toml
project_path = "./my-project"
build_type = "release"
install_dir = "./install"
vendor = false
```

Run with:
```bash
./black-anvil basic-config.toml
```

### Partial Configuration

You can specify only some options. Missing options will trigger interactive prompts:

```toml
# partial-config.toml
build_type = "release"
install_dir = "/usr/local/bin"
# project_path and vendor will be prompted
```

## Configuration Options

### `project_path` (Optional)
- **Type**: String
- **Description**: Path to the Rust project directory
- **Default**: Current directory if not specified (will prompt)
- **Examples**: 
  - `"."` - current directory
  - `"./my-project"` - relative path
  - `"/home/user/rust-app"` - absolute path

### `build_type` (Optional)
- **Type**: String
- **Description**: Build profile to use
- **Valid Values**: `"release"` or `"debug"`
- **Default**: Will prompt if not specified
- **Recommendations**:
  - Use `"release"` for production installations
  - Use `"debug"` for development or testing

### `install_dir` (Optional)
- **Type**: String
- **Description**: Directory where the compiled binary will be installed
- **Default**: `"./install"` if not specified (will prompt)
- **Notes**: Directory will be created if it doesn't exist
- **Examples**:
  - `"./bin"` - local bin directory
  - `"/usr/local/bin"` - system-wide installation (requires permissions)
  - `"~/.local/bin"` - user-local installation

### `vendor` (Optional)
- **Type**: Boolean
- **Description**: Whether to vendor dependencies
- **Default**: `false` if not specified (will prompt)
- **When to use**:
  - `true`: For offline installations, reproducible builds, or air-gapped environments
  - `false`: For normal installations where internet access is available

## Advanced Usage

### Multiple Configuration Files

You can maintain different configuration files for different scenarios:

```bash
# Development installation
./black-anvil configs/dev.toml

# Production installation  
./black-anvil configs/prod.toml

# Testing installation
./black-anvil configs/test.toml
```

### System-wide Installation

To install binaries system-wide (requires appropriate permissions):

```toml
# system-install.toml
project_path = "."
build_type = "release"
install_dir = "/usr/local/bin"
vendor = false
```

```bash
sudo ./black-anvil system-install.toml
```

### Vendored Dependencies

For environments without internet access:

```toml
# offline-install.toml
project_path = "./my-app"
build_type = "release"
install_dir = "./portable-install"
vendor = true
```

This creates a `vendor` directory alongside your binary with all dependencies.

### Scripting and Automation

Black Anvil works well in scripts and CI/CD pipelines:

```bash
#!/bin/bash
# install-script.sh

# Create config on the fly
cat > install-config.toml << EOF
project_path = "$1"
build_type = "release"
install_dir = "$2"
vendor = false
EOF

# Run installation
./black-anvil install-config.toml

# Clean up
rm install-config.toml
```

Usage:
```bash
./install-script.sh ./my-rust-app ./bin
```

## Best Practices

### 1. Use Configuration Files for Repeatable Builds
Instead of manually entering the same information repeatedly, create configuration files for common scenarios.

### 2. Choose Appropriate Build Types
- **Development**: Use `debug` builds for faster compilation during development
- **Production**: Always use `release` builds for better performance

### 3. Verify Installation Directories
- Ensure you have write permissions to the target directory
- For system-wide installations, use `sudo` with caution
- Consider user-local directories like `~/.local/bin` for personal tools

### 4. Use Vendoring Appropriately
- Enable vendoring for offline environments or when dependency consistency is critical
- Disable vendoring for normal development to save disk space and compilation time

### 5. Validate Before Installation
- Test your Rust project builds correctly with `cargo build` before using Black Anvil
- Ensure all dependencies resolve and the project compiles successfully

### 6. Organize Configuration Files
```
configs/
├── dev.toml          # Development builds
├── prod.toml         # Production builds
├── test.toml         # Testing builds
└── offline.toml      # Offline/vendored builds
```

### 7. Document Your Configurations
Add comments to configuration files:

```toml
# Production installation configuration
# Used for deploying release builds to production servers

project_path = "./server-app"
build_type = "release"      # Optimized for performance
install_dir = "/opt/myapp"  # Standard application directory
vendor = true               # Ensure reproducible builds
```

This approach makes your installation process documented, repeatable, and maintainable.