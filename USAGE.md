# Black Anvil Usage Guide

This guide provides comprehensive instructions for using Black Anvil in various scenarios, from simple interactive builds to complex automated deployments with Starlark scripting and Tauri GUI support.

## Table of Contents

- [Getting Started](#getting-started)
- [Interactive Mode](#interactive-mode)
- [Configuration File Mode](#configuration-file-mode)
- [Tauri GUI Interface](#tauri-gui-interface)
- [Starlark Scripting](#starlark-scripting)
- [Advanced Features](#advanced-features)
- [Integration Examples](#integration-examples)
- [Best Practices](#best-practices)

## Getting Started

### Basic Usage Flow

Black Anvil follows a simple workflow:

1. **Project Analysis**: Analyzes your Rust project structure
2. **Build Configuration**: Sets up build parameters (debug/release)
3. **Compilation**: Runs `cargo build` with specified options
4. **Installation**: Copies the binary to the target location
5. **Dependency Management**: Optionally vendors dependencies for offline use

### Prerequisites Check

Before using Black Anvil, verify your setup:

```bash
# Check Rust installation
rustc --version
cargo --version

# Verify you have a Rust project
ls Cargo.toml  # Should exist in your project directory

# Test build capability
cargo check    # Should complete without errors
```

### Usage Modes

Black Anvil offers three interaction modes:

1. **CLI Interactive Mode**: Command-line prompts (fastest to start)
2. **Configuration File Mode**: TOML-based automation (best for CI/CD)
3. **Tauri GUI Mode**: Modern web interface (most user-friendly)

## Interactive Mode

Interactive mode guides you through the build and installation process with prompts.

### Starting Interactive Mode

```bash
./black-anvil
```

### Interactive Walkthrough

#### 1. Project Path Selection

```
Path to Rust project [.]:
```

- **Default**: Current directory (`.`)
- **Custom Path**: Enter absolute or relative path to your Rust project
- **Validation**: Black Anvil checks for `Cargo.toml` file

**Examples:**
```
/home/user/my-rust-project
../another-project
./subproject
```

#### 2. Build Type Selection

```
Build type:
❯ release
  debug
```

- **Release**: Optimized builds for production (`cargo build --release`)
  - Slower compilation, faster execution
  - Smaller binary size
  - No debug symbols
  - Perfect for deployment

- **Debug**: Unoptimized builds for development (`cargo build`)
  - Faster compilation, slower execution
  - Larger binary size
  - Includes debug symbols
  - Ideal for testing and debugging

#### 3. Installation Directory

```
Installation directory [./install]:
```

- **Default**: `./install` directory in current location
- **Common Options**:
  - `~/.local/bin` - User-local installation
  - `/usr/local/bin` - System-wide installation (requires sudo)
  - `/opt/myapp` - Custom application directory
  - `./dist` - Distribution directory

**Platform-specific suggestions:**
- **Linux/macOS**: `/usr/local/bin`, `~/.local/bin`
- **Windows**: `C:\Program Files\MyApp`, `%LOCALAPPDATA%\Programs`

#### 4. Dependency Vendoring

```
Vendor dependencies? [y/N]:
```

- **Yes (y)**: Creates a `vendor/` directory with all dependencies
  - Enables offline installations
  - Ensures reproducible builds
  - Increases installation size
  - Perfect for air-gapped environments

- **No (N)**: Standard installation without dependencies
  - Smaller installation footprint
  - Requires internet for future builds
  - Standard deployment option

### Example Interactive Session

```bash
$ ./black-anvil
Path to Rust project [.]: /home/user/my-cli-tool
Build type:
❯ release
  debug
Installation directory [./install]: ~/.local/bin
Vendor dependencies? [y/N]: y

Building project...
   Compiling my-cli-tool v0.1.0 (/home/user/my-cli-tool)
    Finished release [optimized] target(s) in 45.23s

Vendoring dependencies...
    Vendoring 142 dependencies into vendor/

Installed "my-cli-tool" to /home/user/.local/bin/my-cli-tool
```

## Configuration File Mode

Configuration files enable automated, reproducible builds perfect for CI/CD pipelines.

### Basic Configuration

Create a `config.toml` file:

```toml
# Basic configuration
project_path = "."
build_type = "release"
install_dir = "./install"
vendor = false
```

Run with configuration:

```bash
./black-anvil config.toml
```

### Configuration File Examples

#### Development Configuration

```toml
# dev-config.toml
project_path = "."
build_type = "debug"
install_dir = "~/.local/bin"
vendor = false
```

Perfect for rapid development iterations with fast builds and local installation.

#### Production Configuration

```toml
# prod-config.toml  
project_path = "."
build_type = "release"
install_dir = "/usr/local/bin"
vendor = true
```

Optimized builds with vendored dependencies for production deployment.

#### CI/CD Configuration

```toml
# ci-config.toml
project_path = "/workspace/project"
build_type = "release"
install_dir = "/workspace/dist"
vendor = true
```

Perfect for containerized builds with predictable paths and offline dependencies.

#### Partial Configuration

You can specify only some options. Missing options will trigger interactive prompts:

```toml
# partial-config.toml
build_type = "release"
install_dir = "/usr/local/bin"
# project_path and vendor will be prompted
```

### Using Configuration Files

```bash
# Use specific configuration
./black-anvil dev-config.toml

# Different configurations for different environments
./black-anvil configs/development.toml
./black-anvil configs/staging.toml
./black-anvil configs/production.toml
```

## Tauri GUI Interface

The Tauri frontend provides a modern, user-friendly graphical interface for Black Anvil.

### Starting the GUI

```bash
cd tauri-app

# Development mode
npm run tauri dev

# Or build and run
npm run tauri build
```

### GUI Features

#### Project Selection
- **Browse Button**: Graphical directory picker
- **Recent Projects**: Quick access to previously used projects
- **Validation**: Real-time feedback on project validity

#### Build Configuration
- **Build Type Toggle**: Visual selection between debug/release
- **Progress Indicators**: Real-time build progress
- **Output Display**: Formatted cargo output with syntax highlighting

#### Installation Options
- **Directory Picker**: Graphical installation directory selection
- **Permission Warnings**: Alerts for directories requiring elevated permissions
- **Space Estimation**: Preview of installation size requirements

#### Advanced Settings
- **Vendor Toggle**: Clear option for dependency vendoring
- **Configuration Profiles**: Save and load configuration presets
- **Environment Variables**: Set custom build environment

### GUI Workflow Example

1. **Launch Application**
   ```bash
   cd tauri-app && npm run tauri dev
   ```

2. **Select Project**
   - Click "Browse" to select project directory
   - Or choose from recent projects list

3. **Configure Build**
   - Toggle between Debug/Release builds
   - Review build estimation (time, size)

4. **Choose Installation**
   - Select installation directory
   - Review permission requirements
   - Enable vendoring if needed

5. **Execute Build**
   - Click "Build & Install"
   - Monitor progress in real-time
   - Review completion summary

## Starlark Scripting

Black Anvil includes embedded Starlark scripting support for advanced automation and custom build logic.

### What is Starlark?

Starlark is a Python-like configuration language that's:
- **Safe**: Deterministic and sandboxed
- **Simple**: Easy to learn Python-subset syntax
- **Fast**: Efficient execution for build scripts

### Using Starlark Scripts

#### Basic Script Execution

```python
# build-script.star
def configure_build():
    return {
        "build_type": "release",
        "features": ["cli", "web"],
        "target": "x86_64-unknown-linux-gnu"
    }

# Script returns configuration
configure_build()
```

#### Integration with Black Anvil

```toml
# config.toml with Starlark
project_path = "."
build_type = "release"
install_dir = "./install"
vendor = false

# Reference Starlark script for advanced logic
[starlark]
script = "build-scripts/custom-build.star"
```

#### Advanced Starlark Examples

**Conditional Build Configuration**:
```python
# conditional-build.star
def select_build_type(is_production):
    if is_production:
        return "release"
    else:
        return "debug"

def configure_installation(platform):
    if platform == "linux":
        return "/usr/local/bin"
    elif platform == "windows":
        return "C:\\Program Files\\MyApp"
    else:
        return "./install"

# Main configuration function
def main():
    is_prod = env.get("PRODUCTION", "false") == "true"
    platform = env.get("TARGET_PLATFORM", "linux")
    
    return {
        "build_type": select_build_type(is_prod),
        "install_dir": configure_installation(platform),
        "vendor": is_prod
    }

main()
```

**Feature Flag Management**:
```python
# features.star
def select_features(target_env):
    base_features = ["core"]
    
    if target_env == "server":
        base_features.extend(["api", "database"])
    elif target_env == "desktop":
        base_features.extend(["gui", "local-storage"])
    elif target_env == "embedded":
        base_features.extend(["minimal", "no-std"])
    
    return base_features

def main():
    target = env.get("TARGET_ENV", "desktop")
    features = select_features(target)
    
    return {
        "build_type": "release",
        "features": features,
        "no_default_features": target == "embedded"
    }

main()
```

### Starlark API Reference

**Available Functions**:
- `env.get(key, default)` - Get environment variable
- `path.join(*parts)` - Join path components
- `os.platform()` - Get current platform
- `time.now()` - Current timestamp

**Available Modules**:
- `json` - JSON parsing and serialization
- `base64` - Base64 encoding/decoding
- `re` - Regular expressions
- `time` - Time utilities

## Advanced Features

### Dependency Vendoring

Dependency vendoring creates offline-capable installations by bundling all required dependencies.

#### When to Use Vendoring

- **Air-gapped environments** without internet access
- **Reproducible builds** with locked dependency versions
- **Portable installations** that work without cargo
- **Security-conscious environments** avoiding external downloads

#### Vendoring Process

```bash
# Manual vendoring (optional - Black Anvil does this automatically)
cd your-project
cargo vendor

# Results in vendor/ directory with all dependencies
ls vendor/
```

#### Vendoring Configuration

```toml
# Enable vendoring in configuration
vendor = true

# Results in installation like:
# install_dir/
# ├── your-binary
# └── vendor/
#     ├── dependency1/
#     ├── dependency2/
#     └── ...
```

### Cross-Platform Considerations

#### Path Handling

Black Anvil automatically handles platform-specific paths:

```toml
# Works on all platforms
install_dir = "~/.local/bin"        # User directory
install_dir = "/tmp/install"        # Unix-style (auto-converted on Windows)
install_dir = "C:\\Program Files"   # Windows-style (works on Windows only)
```

#### Binary Extensions

Binary names are automatically adjusted per platform:
- **Linux/macOS**: `my-tool`
- **Windows**: `my-tool.exe`

#### Permission Handling

- **Unix systems**: May require `sudo` for system directories
- **Windows**: May trigger UAC prompts for protected directories
- **User directories**: Generally work without elevation

## Integration Examples

### CI/CD Pipeline Integration

#### GitHub Actions

```yaml
name: Build and Deploy
on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          
      - name: Build with Black Anvil
        run: |
          git clone https://github.com/Big-Sky-Tech/Black-Anvil.git
          cd Black-Anvil && cargo build --release
          ./target/release/black-anvil ../ci-config.toml
          
      - name: Upload artifacts
        uses: actions/upload-artifact@v2
        with:
          name: built-binary
          path: ./install/
```

#### Docker Integration

```dockerfile
FROM rust:1.70

# Install Black Anvil
RUN git clone https://github.com/Big-Sky-Tech/Black-Anvil.git && \
    cd Black-Anvil && \
    cargo build --release

# Copy project and configuration
COPY . /workspace
COPY docker-config.toml /workspace/

# Build and install
WORKDIR /workspace
RUN /Black-Anvil/target/release/black-anvil docker-config.toml

# Final runtime image
FROM debian:slim
COPY --from=0 /workspace/install/ /usr/local/bin/
```

### Script Integration

#### Batch Processing

```bash
#!/bin/bash
# build-all.sh - Build multiple projects

projects=("project1" "project2" "project3")
configs=("dev-config.toml" "test-config.toml" "prod-config.toml")

for project in "${projects[@]}"; do
    echo "Building $project..."
    cd "$project"
    
    for config in "${configs[@]}"; do
        echo "  Using $config..."
        ../black-anvil "../configs/$config"
    done
    
    cd ..
done
```

## Best Practices

### Configuration Management

#### Version Control

```gitignore
# Don't commit installation directories
install/
dist/
portable-app/

# Don't commit vendored dependencies
vendor/

# Do commit configuration files
configs/*.toml
scripts/*.star
```

#### Configuration Templates

Create template configurations for common scenarios:

```toml
# template-dev.toml
project_path = "."           # Current project
build_type = "debug"         # Fast builds
install_dir = "~/.local/bin" # User-local
vendor = false               # No vendoring for dev
```

#### Environment-Specific Configs

Organize configurations by environment:

```
configs/
├── development.toml
├── testing.toml
├── staging.toml
└── production.toml

scripts/
├── development.star
├── feature-flags.star
└── deployment.star
```

### Performance Optimization

#### Build Caching

Leverage Rust's incremental compilation:

```bash
# Set cargo cache directory
export CARGO_HOME="/shared/cargo"

# Builds will reuse compilation cache
./black-anvil config.toml
```

#### Parallel Builds

Rust automatically uses parallel compilation, but you can optimize:

```bash
# Set number of parallel jobs
export CARGO_BUILD_JOBS=8

# Use with Black Anvil
./black-anvil config.toml
```

### Security Considerations

#### Path Validation

Always validate paths in configuration files:

```toml
# Good: Explicit, safe paths
install_dir = "/opt/myapp"

# Avoid: Relative paths that could escape
install_dir = "../../system/bin"
```

#### Starlark Script Security

Starlark scripts run in a safe sandbox, but follow best practices:

```python
# Good: Use environment variables for sensitive data
password = env.get("DB_PASSWORD", "")

# Avoid: Hardcoded secrets
password = "secret123"  # Don't do this
```

This completes the comprehensive usage guide for Black Anvil with its advanced features including Starlark scripting and Tauri GUI support.