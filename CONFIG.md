# Black Anvil Configuration Reference

This document provides a comprehensive reference for Black Anvil configuration files.

## Table of Contents

- [Configuration File Format](#configuration-file-format)
- [Configuration Fields](#configuration-fields)
- [Example Configurations](#example-configurations)
- [Validation Rules](#validation-rules)
- [Error Handling](#error-handling)

## Configuration File Format

Black Anvil uses TOML (Tom's Obvious, Minimal Language) for configuration files. TOML is a human-readable configuration file format that's easy to read and write.

### Basic Structure

```toml
# Configuration for Black Anvil
# All fields are optional

project_path = "path/to/project"
build_type = "release"
install_dir = "/path/to/install"
vendor = false
```

### File Extension

Configuration files should use the `.toml` extension, though this is not strictly required.

## Configuration Fields

### `project_path`

**Type**: String (Optional)  
**Description**: Specifies the path to the Rust project directory to build and install.

```toml
project_path = "./my-project"
```

**Valid Values**:
- Relative paths: `"."`, `"./subdir"`, `"../other-project"`
- Absolute paths: `"/home/user/project"`, `"C:\\Users\\user\\project"` (Windows)
- Home directory: `"~/my-project"` (Unix-like systems)

**Default Behavior**: If not specified, Black Anvil will prompt interactively.

**Requirements**:
- Path must exist
- Must contain a valid `Cargo.toml` file
- Must be a valid Rust project that can be built with `cargo build`

**Examples**:
```toml
# Current directory
project_path = "."

# Relative path
project_path = "./rust-cli-tool"

# Absolute path (Unix)
project_path = "/home/developer/my-rust-app"

# Absolute path (Windows)
project_path = "C:\\Projects\\my-rust-app"
```

### `build_type`

**Type**: String (Optional)  
**Description**: Specifies the Cargo build profile to use.

```toml
build_type = "release"
```

**Valid Values**:
- `"release"`: Optimized build with optimizations enabled
- `"debug"`: Debug build with debug symbols and no optimizations

**Default Behavior**: If not specified, Black Anvil will prompt with a selection menu.

**Characteristics**:

| Build Type | Optimization | Debug Symbols | Compilation Speed | Binary Size | Performance |
|------------|--------------|---------------|-------------------|-------------|-------------|
| `release`  | Yes          | No            | Slower            | Smaller     | Faster      |
| `debug`    | No           | Yes           | Faster            | Larger      | Slower      |

**Examples**:
```toml
# For production use
build_type = "release"

# For development/testing
build_type = "debug"
```

### `install_dir`

**Type**: String (Optional)  
**Description**: Specifies the directory where the compiled binary will be copied.

```toml
install_dir = "/usr/local/bin"
```

**Default Behavior**: If not specified, defaults to `"./install"` (will prompt interactively).

**Requirements**:
- Directory will be created if it doesn't exist
- Must have write permissions to the directory (or parent directory if creating)
- Sufficient disk space for the binary

**Common Installation Locations**:

| Location | Description | Use Case |
|----------|-------------|----------|
| `"./install"` | Local install directory | Development, testing |
| `"/usr/local/bin"` | System-wide binaries | System administrators |
| `"~/.local/bin"` | User-local binaries | Personal tools |
| `"./bin"` | Project bin directory | Project-specific builds |
| `"/opt/myapp"` | Application directory | Application deployments |

**Examples**:
```toml
# Local development
install_dir = "./build/bin"

# User-local installation
install_dir = "~/.local/bin"

# System-wide installation (requires sudo)
install_dir = "/usr/local/bin"

# Windows user directory
install_dir = "C:\\Users\\username\\bin"

# Application-specific directory
install_dir = "/opt/mycompany/myapp"
```

### `vendor`

**Type**: Boolean (Optional)  
**Description**: Controls whether to vendor (download and store locally) the project's dependencies.

```toml
vendor = true
```

**Valid Values**:
- `true`: Enable dependency vendoring
- `false`: Disable dependency vendoring

**Default Behavior**: If not specified, defaults to `false` (will prompt interactively).

**When Vendoring is Useful**:
- Offline installations
- Air-gapped environments
- Reproducible builds
- Avoiding dependency resolution during deployment
- Compliance requirements for dependency auditing

**Vendoring Process**:
1. Runs `cargo vendor` in the project directory
2. Creates a `vendor` directory with all dependencies
3. Copies the vendor directory to the installation directory

**Disk Space Considerations**:
- Vendoring can significantly increase installation size
- Dependencies are stored in uncompressed source form
- Consider available disk space when enabling

**Examples**:
```toml
# Enable vendoring for offline deployment
vendor = true

# Disable vendoring for development
vendor = false
```

## Example Configurations

### Development Configuration

```toml
# dev-config.toml
# Configuration for development builds

project_path = "."
build_type = "debug"
install_dir = "./dev-install"
vendor = false
```

**Use Case**: Quick development builds with debug symbols for local testing.

### Production Configuration

```toml
# prod-config.toml
# Configuration for production deployment

project_path = "./server-application"
build_type = "release"
install_dir = "/opt/mycompany/myapp"
vendor = true
```

**Use Case**: Optimized production deployment with vendored dependencies.

### User Installation Configuration

```toml
# user-config.toml
# Configuration for user-local installation

project_path = "."
build_type = "release"
install_dir = "~/.local/bin"
vendor = false
```

**Use Case**: Installing personal tools without system-wide access.

### Continuous Integration Configuration

```toml
# ci-config.toml
# Configuration for CI/CD pipelines

project_path = "./project"
build_type = "release"
install_dir = "./artifacts/bin"
vendor = true
```

**Use Case**: Automated builds in CI/CD with consistent dependency management.

### Minimal Configuration

```toml
# minimal-config.toml
# Only specify build type, prompt for everything else

build_type = "release"
```

**Use Case**: Consistent build type with flexibility for other options.

### Offline Configuration

```toml
# offline-config.toml
# Configuration for environments without internet access

project_path = "./application"
build_type = "release"
install_dir = "./offline-install"
vendor = true
```

**Use Case**: Deployments in environments without internet connectivity.

## Validation Rules

### File Validation
- Configuration file must be valid TOML format
- Unknown fields are ignored (for forward compatibility)
- Invalid TOML syntax will result in parsing errors

### Field Validation

**`project_path`**:
- Must be a valid file system path
- Directory must exist and be accessible
- Must contain a `Cargo.toml` file
- Rust project must be buildable

**`build_type`**:
- Must be exactly `"release"` or `"debug"`
- Case-sensitive
- No other values accepted

**`install_dir`**:
- Must be a valid file system path
- Parent directory must exist or be creatable
- Must have write permissions

**`vendor`**:
- Must be a boolean value (`true` or `false`)
- String values like `"true"` or `"false"` are not accepted

## Error Handling

### Common Configuration Errors

**Invalid TOML Syntax**:
```
Error: Failed to parse config
Caused by: invalid TOML syntax at line 3, column 15
```
**Solution**: Check TOML syntax, ensure proper quoting and formatting.

**Invalid Build Type**:
```toml
# ❌ Invalid
build_type = "Release"  # Case-sensitive, should be "release"
build_type = "opt"      # Invalid value

# ✅ Valid
build_type = "release"
build_type = "debug"
```

**Invalid Boolean**:
```toml
# ❌ Invalid
vendor = "true"   # String, not boolean
vendor = 1        # Number, not boolean

# ✅ Valid
vendor = true
vendor = false
```

**Non-existent Project Path**:
```
Error: Failed to read /path/to/project/Cargo.toml
Caused by: No such file or directory
```
**Solution**: Verify the project path exists and contains a `Cargo.toml` file.

**Permission Errors**:
```
Error: Failed to create directory /usr/local/bin
Caused by: Permission denied
```
**Solution**: Ensure write permissions or run with appropriate privileges (e.g., `sudo`).

### Best Practices for Error Prevention

1. **Validate paths**: Ensure all paths exist and are accessible
2. **Test configurations**: Run with test projects before production use
3. **Use relative paths**: When possible, use relative paths for portability
4. **Check permissions**: Verify write access to installation directories
5. **Lint TOML**: Use TOML linters or validators to check syntax

### Configuration File Templates

Keep template configurations for common scenarios:

```toml
# template-dev.toml
project_path = "REPLACE_WITH_PROJECT_PATH"
build_type = "debug"
install_dir = "./dev-install"
vendor = false
```

```toml
# template-prod.toml
project_path = "REPLACE_WITH_PROJECT_PATH"
build_type = "release"
install_dir = "REPLACE_WITH_INSTALL_DIR"
vendor = true
```

This reference should help you create effective Black Anvil configurations for any use case.