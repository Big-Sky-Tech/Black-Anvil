# Black Anvil Configuration Examples

This directory contains working example configurations for Black Anvil covering common usage scenarios. All examples have been tested and verified to work with the current Black Anvil implementation featuring Starlark scripting and Tauri GUI support.

## Quick Reference

| Configuration | Use Case | Build Type | Vendoring | Target |
|---------------|----------|------------|-----------|---------|
| [dev-config.toml](#development) | Development builds | Debug | No | Local user bin |
| [prod-config.toml](#production) | Production deployment | Release | Yes | System-wide |
| [user-install.toml](#user-installation) | Personal tools | Release | No | User directory |
| [system-install.toml](#system-installation) | System utilities | Release | No | System directory |
| [offline-install.toml](#offline-deployment) | Air-gapped deployment | Release | Yes | Custom directory |
| [ci-config.toml](#cicd-pipeline) | CI/CD builds | Release | Yes | Artifacts directory |
| [portable-install.toml](#portable-application) | Portable apps | Release | Yes | Self-contained |
| [build-only.toml](#partial-configuration) | Partial config | Release | Prompt | Interactive |
| [location-only.toml](#minimal-configuration) | Minimal config | Prompt | Prompt | Specific directory |

## Configuration Examples

### Development Configuration

**File: `dev-config.toml`**

Fast development builds for rapid iteration and testing.

**Use Case**: Daily development work with fast compilation times and local installation.

**Usage**:
```bash
./black-anvil examples/dev-config.toml
```

### Production Configuration

**File: `prod-config.toml`**

Optimized production deployment with vendored dependencies.

**Use Case**: Production server deployment with maximum performance and offline capability.

**Usage**:
```bash
sudo ./black-anvil examples/prod-config.toml
```

### User Installation Configuration

**File: `user-install.toml`**

User-local installation for personal command-line tools.

**Use Case**: Installing personal tools without system administrator access.

**Setup**: Ensure `~/.local/bin` is in your PATH:
```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

**Usage**:
```bash
./black-anvil examples/user-install.toml
```

### System Installation Configuration

**File: `system-install.toml`**

System-wide installation for utilities used by all users.

**Use Case**: Installing system utilities available to all users on the machine.

**Usage**:
```bash
sudo ./black-anvil examples/system-install.toml
```

### Offline Deployment Configuration

**File: `offline-install.toml`**

Self-contained deployment for air-gapped environments.

**Use Case**: Deployment in environments without internet access or strict security requirements.

**Usage**:
```bash
./black-anvil examples/offline-install.toml
```

**Deployment**: Copy the entire installation directory to target systems.

### CI/CD Pipeline Configuration

**File: `ci-config.toml`**

Automated builds in CI/CD pipelines with predictable paths.

**Use Case**: Containerized builds with artifact collection and reproducible results.

**Usage in CI**:
```bash
./black-anvil examples/ci-config.toml
```

**Docker Integration**:
```dockerfile
COPY ci-config.toml /workspace/
RUN ./black-anvil /workspace/ci-config.toml
```

### Portable Application Configuration

**File: `portable-install.toml`**

Self-contained application packages for distribution.

**Use Case**: Creating portable applications that can be distributed as archives.

**Usage**:
```bash
./black-anvil examples/portable-install.toml
# Creates self-contained directory
```

### Partial Configuration (Interactive Prompts)

**File: `build-only.toml`**

Enforces specific build type while prompting for other options.

**Use Case**: Consistent build quality with flexibility for paths and deployment.

**Usage**:
```bash
./black-anvil examples/build-only.toml
# Will prompt for missing options
```

### Minimal Configuration

**File: `location-only.toml`**

Specifies only installation directory, prompts for everything else.

**Use Case**: Consistent installation location with maximum flexibility.

**Usage**:
```bash
./black-anvil examples/location-only.toml
# Will prompt for project path, build type, and vendoring
```

## Usage Patterns

### Multiple Environments

Deploy to different environments using different configurations:

```bash
#!/bin/bash
# deploy-all.sh - Deploy to multiple environments

environments=("dev" "staging" "prod")

for env in "${environments[@]}"; do
    echo "Deploying to $env environment..."
    ./black-anvil "examples/${env}-config.toml"
done
```

### Conditional Configuration

Choose configuration based on environment variables:

```bash
#!/bin/bash
# conditional-build.sh

case "${ENVIRONMENT:-dev}" in
    "production"|"prod")
        CONFIG="examples/prod-config.toml"
        ;;
    "staging")
        CONFIG="examples/staging-config.toml"
        ;;
    *)
        CONFIG="examples/dev-config.toml"
        ;;
esac

echo "Using configuration: $CONFIG"
./black-anvil "$CONFIG"
```

### With Tauri GUI

All example configurations work with the Tauri frontend:

```bash
# Start the GUI
cd tauri-app
npm run tauri dev

# Load any example configuration through the file picker
# or drag and drop configuration files onto the GUI
```

### With Starlark Scripts

Combine configurations with Starlark scripting for advanced automation:

```python
# advanced-config.star
def generate_config():
    env_type = env.get("ENVIRONMENT", "development")
    
    if env_type == "production":
        return load_config("examples/prod-config.toml")
    else:
        return load_config("examples/dev-config.toml")

generate_config()
```

## Customization Guide

### Creating Custom Configurations

1. **Copy a similar example** as a starting point:
   ```bash
   cp examples/dev-config.toml my-custom-config.toml
   ```

2. **Modify for your environment**:
   - Update `project_path` to your project location
   - Change `install_dir` to your target directory
   - Adjust `build_type` and `vendor` as needed

3. **Test with your project**:
   ```bash
   ./black-anvil my-custom-config.toml
   ```

### Common Customizations

**Platform-specific paths**:
```toml
# Linux/macOS
install_dir = "/usr/local/bin"

# Windows
install_dir = "C:\\Program Files\\MyApp"

# Cross-platform user directory
install_dir = "~/.local/bin"
```

**Application-specific directories**:
```toml
# Company/application directory
install_dir = "/opt/mycompany/myapp"

# User application directory
install_dir = "~/Applications/MyApp"

# Project-relative directory
install_dir = "./dist/bin"
```

## Testing and Validation

All examples have been tested with:

- **Rust**: 1.70+
- **Cargo**: 1.70+
- **Black Anvil**: Latest fast-track branch
- **Platforms**: Linux, macOS, Windows

### Test Script

```bash
#!/bin/bash
# test-examples.sh - Validate all configurations

for config in examples/*.toml; do
    echo "Testing $(basename "$config")..."
    if ./black-anvil "$config"; then
        echo "✅ Success"
    else
        echo "❌ Failed"
    fi
done
```

## Integration with Advanced Features

### Starlark Scripting

Use configurations as templates in Starlark scripts:

```python
# config-generator.star
def customize_config(base_config):
    # Load base configuration
    config = load_config(base_config)
    
    # Customize based on environment
    if env.get("PRODUCTION") == "true":
        config["vendor"] = True
        config["build_type"] = "release"
    
    return config

customize_config("examples/dev-config.toml")
```

### Tauri Integration

Load example configurations in the Tauri GUI:

1. **File Menu → Open Configuration**
2. **Select any example .toml file**
3. **Customize settings in the GUI**
4. **Save as new configuration**

## Troubleshooting

### Common Issues

**Permission denied**:
- Use `sudo` for system-wide installations
- Choose user directories for personal tools
- Verify directory write permissions

**Build failures**:
- Test with `cargo build` first
- Check Rust toolchain version
- Verify project dependencies

**Path not found**:
- Use absolute paths in CI environments
- Ensure `Cargo.toml` exists in project path
- Check directory permissions

For detailed troubleshooting, see:
- [USAGE.md](../USAGE.md) - Comprehensive usage guide
- [CONFIG.md](../CONFIG.md) - Configuration reference
- [README.md](../README.md) - Main documentation

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