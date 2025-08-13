# Black Anvil Examples

This directory contains example configuration files and use cases for Black Anvil.

## Example Configuration Files

### Basic Examples

- [`dev-config.toml`](dev-config.toml) - Development build configuration
- [`prod-config.toml`](prod-config.toml) - Production build configuration  
- [`user-install.toml`](user-install.toml) - User-local installation
- [`system-install.toml`](system-install.toml) - System-wide installation

### Advanced Examples

- [`offline-install.toml`](offline-install.toml) - Offline installation with vendored dependencies
- [`ci-config.toml`](ci-config.toml) - Continuous integration configuration
- [`portable-install.toml`](portable-install.toml) - Portable installation for distribution

### Partial Configurations

- [`build-only.toml`](build-only.toml) - Only specifies build type
- [`location-only.toml`](location-only.toml) - Only specifies installation directory

## Usage

Copy any example configuration file and modify it for your needs:

```bash
# Copy and edit an example
cp examples/dev-config.toml my-config.toml
# Edit my-config.toml with your specific paths

# Use with Black Anvil
./black-anvil my-config.toml
```

## Common Use Cases

### 1. Development Workflow

Use `dev-config.toml` for fast development builds:

```bash
./black-anvil examples/dev-config.toml
```

### 2. Production Deployment

Use `prod-config.toml` for optimized production builds:

```bash
./black-anvil examples/prod-config.toml
```

### 3. Installing Personal Tools

Use `user-install.toml` to install tools in your home directory:

```bash
./black-anvil examples/user-install.toml
```

### 4. System Administration

Use `system-install.toml` for system-wide installations:

```bash
sudo ./black-anvil examples/system-install.toml
```

### 5. Offline Environments

Use `offline-install.toml` for environments without internet access:

```bash
./black-anvil examples/offline-install.toml
```

## Customization Tips

1. **Modify paths**: Update `project_path` and `install_dir` for your environment
2. **Choose build type**: Use `debug` for development, `release` for production  
3. **Consider vendoring**: Enable for offline use or reproducible builds
4. **Test first**: Always test configurations with non-critical projects first

## Creating Your Own Configurations

Start with an example that matches your use case, then customize:

1. Copy the closest matching example
2. Update the paths for your environment
3. Adjust the build type and vendoring as needed
4. Test with a sample project
5. Document your configuration for future use

For more detailed information, see:
- [USAGE.md](../USAGE.md) - Detailed usage guide
- [CONFIG.md](../CONFIG.md) - Configuration reference
- [README.md](../README.md) - General documentation