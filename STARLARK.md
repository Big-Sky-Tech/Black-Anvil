# Starlark Scripting Guide for Black Anvil

Black Anvil includes embedded Starlark scripting support, allowing you to create sophisticated build automation and configuration logic that goes beyond simple TOML files.

## Table of Contents

- [Introduction to Starlark](#introduction-to-starlark)
- [Getting Started](#getting-started)
- [Starlark in Black Anvil](#starlark-in-black-anvil)
- [API Reference](#api-reference)
- [Examples](#examples)
- [Best Practices](#best-practices)
- [Advanced Use Cases](#advanced-use-cases)

## Introduction to Starlark

Starlark is a Python-like configuration language designed for embedding in applications. It's:

- **Safe**: Deterministic execution with no system access
- **Simple**: Python-subset syntax that's easy to learn
- **Fast**: Efficient execution for build scripts
- **Hermetic**: No global state, perfect for reproducible builds

### Why Starlark for Build Configuration?

- **Conditional Logic**: Make decisions based on environment, platform, or project state
- **Code Reuse**: Functions and modules for common configuration patterns
- **Dynamic Configuration**: Generate configuration based on runtime conditions
- **Complex Logic**: Beyond what's possible with static TOML files

## Getting Started

### Basic Starlark Script

Create a simple Starlark script:

```python
# hello.star
def main():
    return "Hello from Starlark!"

main()
```

### Testing Starlark Scripts

Black Anvil provides a built-in function to execute Starlark scripts:

```rust
use black_anvil::run_starlark;

let script = r#"
def greet(name):
    return "Hello, " + name + "!"

greet("World")
"#;

let result = run_starlark(script)?;
println!("Result: {}", result); // "Hello, World!"
```

## Starlark in Black Anvil

### Integration Patterns

Black Anvil can use Starlark scripts in several ways:

#### 1. Configuration Generation

```python
# config-generator.star
def generate_config(env):
    """Generate build configuration based on environment."""
    if env == "production":
        return {
            "build_type": "release",
            "install_dir": "/opt/myapp",
            "vendor": True,
            "features": ["production", "optimized"]
        }
    elif env == "development":
        return {
            "build_type": "debug", 
            "install_dir": "./dev-install",
            "vendor": False,
            "features": ["debug", "testing"]
        }
    else:
        return {
            "build_type": "release",
            "install_dir": "./install",
            "vendor": False,
            "features": []
        }

# Usage
env = "production"  # Could come from environment variable
generate_config(env)
```

#### 2. Platform-Specific Configuration

```python
# platform-config.star
def get_platform_config(platform):
    """Generate platform-specific configuration."""
    base_config = {
        "build_type": "release",
        "vendor": True
    }
    
    if platform == "windows":
        base_config.update({
            "install_dir": "C:\\Program Files\\MyApp",
            "binary_suffix": ".exe"
        })
    elif platform == "macos":
        base_config.update({
            "install_dir": "/Applications/MyApp.app/Contents/MacOS",
            "binary_suffix": ""
        })
    else:  # Linux and other Unix-like
        base_config.update({
            "install_dir": "/usr/local/bin",
            "binary_suffix": ""
        })
    
    return base_config

# Auto-detect platform or use environment variable
import os
platform = os.environ.get("TARGET_PLATFORM", "linux")
get_platform_config(platform)
```

#### 3. Feature Flag Management

```python
# features.star
def select_features(target, enable_experimental=False):
    """Select Cargo features based on target and flags."""
    
    # Base features always included
    features = ["core", "logging"]
    
    # Target-specific features
    if target == "server":
        features.extend(["api", "database", "auth"])
    elif target == "cli":
        features.extend(["terminal", "config-file"])
    elif target == "embedded":
        features = ["core"]  # Minimal for embedded
    
    # Optional experimental features
    if enable_experimental:
        features.append("experimental")
    
    # Additional features based on environment
    if os.environ.get("ENABLE_METRICS", "false") == "true":
        features.append("metrics")
    
    return {
        "features": features,
        "no_default_features": target == "embedded"
    }

# Configuration
target = os.environ.get("BUILD_TARGET", "cli")
experimental = os.environ.get("EXPERIMENTAL", "false") == "true"

config = select_features(target, experimental)
config.update({
    "build_type": "release",
    "install_dir": "./install",
    "vendor": False
})

config
```

## API Reference

### Built-in Functions

Black Anvil provides these functions to Starlark scripts:

#### Environment Access

```python
# Get environment variable with optional default
value = env.get("MY_VAR", "default_value")

# Check if environment variable exists
if env.has("DEBUG"):
    print("Debug mode enabled")

# Get all environment variables (as read-only dict)
all_env = env.all()
```

#### Path Operations

```python
# Join path components
full_path = path.join("usr", "local", "bin")  # "/usr/local/bin"

# Get absolute path
abs_path = path.abs("./relative/path")

# Check if path exists
if path.exists("./Cargo.toml"):
    print("Rust project detected")

# Get path basename and dirname
name = path.basename("/usr/local/bin/myapp")  # "myapp"
dir = path.dirname("/usr/local/bin/myapp")    # "/usr/local/bin"
```

#### Platform Detection

```python
# Get current platform
platform = os.platform()  # "linux", "windows", "macos", etc.

# Platform-specific logic
if os.is_windows():
    install_dir = "C:\\Program Files\\MyApp"
elif os.is_macos():
    install_dir = "/Applications"
else:
    install_dir = "/usr/local/bin"
```

#### Time and Date

```python
# Current timestamp
now = time.now()  # Unix timestamp

# Format timestamp
formatted = time.format(now, "%Y-%m-%d %H:%M:%S")

# Time-based configuration
if time.hour() < 6:
    # Night builds get extra debugging
    build_type = "debug"
else:
    build_type = "release"
```

### Available Modules

#### JSON Module

```python
# Parse JSON string
data = json.decode('{"name": "myapp", "version": "1.0"}')
name = data["name"]

# Generate JSON
config = {"build_type": "release", "vendor": True}
json_str = json.encode(config)
```

#### Base64 Module

```python
# Encode string to base64
encoded = base64.encode("Hello World")

# Decode base64 to string
decoded = base64.decode(encoded)
```

#### Regular Expressions

```python
# Match pattern
if re.match(r"^v\d+\.\d+\.\d+$", version):
    print("Valid semantic version")

# Find all matches
versions = re.findall(r"\d+\.\d+\.\d+", changelog_text)

# Replace pattern
cleaned = re.sub(r"[^a-zA-Z0-9]", "_", app_name)
```

## Examples

### Environment-Based Configuration

```python
# env-config.star
def main():
    """Generate configuration based on environment variables."""
    
    # Determine environment
    env_name = env.get("ENVIRONMENT", "development")
    
    # Base configuration
    config = {
        "project_path": ".",
        "vendor": False
    }
    
    # Environment-specific overrides
    if env_name == "production":
        config.update({
            "build_type": "release",
            "install_dir": "/opt/myapp",
            "vendor": True
        })
    elif env_name == "staging":
        config.update({
            "build_type": "release", 
            "install_dir": "./staging-install",
            "vendor": True
        })
    else:  # development
        config.update({
            "build_type": "debug",
            "install_dir": "~/.local/bin",
            "vendor": False
        })
    
    # Override with specific environment variables
    if env.has("BUILD_TYPE"):
        config["build_type"] = env.get("BUILD_TYPE")
    
    if env.has("INSTALL_DIR"):
        config["install_dir"] = env.get("INSTALL_DIR")
    
    return config

main()
```

### Cross-Platform Build Script

```python
# cross-platform.star
def get_cross_platform_config():
    """Generate platform-specific build configuration."""
    
    platform = os.platform()
    arch = env.get("TARGET_ARCH", "x86_64")
    
    config = {
        "build_type": "release",
        "vendor": True,
        "project_path": "."
    }
    
    # Platform-specific installation directories
    if platform == "windows":
        config["install_dir"] = "C:\\Program Files\\MyApp"
        config["binary_name"] = "myapp.exe"
    elif platform == "macos":
        config["install_dir"] = "/usr/local/bin"
        config["binary_name"] = "myapp"
        # macOS-specific build flags
        config["cargo_flags"] = ["--target", arch + "-apple-darwin"]
    else:  # Linux and other Unix
        config["install_dir"] = "/usr/local/bin"
        config["binary_name"] = "myapp"
        # Linux-specific build flags
        config["cargo_flags"] = ["--target", arch + "-unknown-linux-gnu"]
    
    return config

get_cross_platform_config()
```

### CI/CD Pipeline Configuration

```python
# ci-config.star
def generate_ci_config():
    """Generate configuration for CI/CD pipeline."""
    
    # Detect CI environment
    is_ci = env.get("CI", "false") == "true"
    branch = env.get("GITHUB_REF_NAME", "main")
    is_release = branch.startswith("release/") or branch == "main"
    
    config = {
        "project_path": ".",
        "vendor": True  # Always vendor in CI for reproducibility
    }
    
    if is_release:
        config.update({
            "build_type": "release",
            "install_dir": "./artifacts/release"
        })
    else:
        config.update({
            "build_type": "debug",
            "install_dir": "./artifacts/debug"
        })
    
    # Add build metadata
    if is_ci:
        build_number = env.get("GITHUB_RUN_NUMBER", "unknown")
        commit_sha = env.get("GITHUB_SHA", "unknown")[:8]
        
        config["build_metadata"] = {
            "build_number": build_number,
            "commit_sha": commit_sha,
            "branch": branch,
            "timestamp": time.format(time.now(), "%Y-%m-%d %H:%M:%S")
        }
    
    return config

generate_ci_config()
```

### Conditional Feature Selection

```python
# feature-flags.star
def select_features():
    """Select Cargo features based on various conditions."""
    
    # Base features
    features = ["core"]
    
    # Environment-based features
    if env.get("ENABLE_GUI", "false") == "true":
        features.append("gui")
    
    if env.get("ENABLE_API", "true") == "true":
        features.append("api")
    
    # Platform-specific features
    platform = os.platform()
    if platform == "linux":
        features.append("unix-signals")
    elif platform == "windows":
        features.append("windows-service")
    
    # Target-specific features
    target = env.get("BUILD_TARGET", "default")
    if target == "server":
        features.extend(["database", "auth", "metrics"])
    elif target == "embedded":
        # Minimal feature set for embedded
        features = ["core", "no-std"]
    
    # Development vs production
    if env.get("DEVELOPMENT", "false") == "true":
        features.append("dev-tools")
    
    return {
        "features": features,
        "build_type": "release" if target == "embedded" else "debug",
        "no_default_features": target == "embedded"
    }

select_features()
```

## Best Practices

### 1. Keep Scripts Focused

Write single-purpose scripts that do one thing well:

```python
# Good: Focused script
def select_install_directory():
    """Select installation directory based on platform and user."""
    # ... implementation
    return install_dir

# Avoid: Monolithic scripts that do everything
```

### 2. Use Environment Variables for Input

Make scripts configurable through environment variables:

```python
# Good: Configurable through environment
debug_mode = env.get("DEBUG", "false") == "true"
target_platform = env.get("TARGET_PLATFORM", os.platform())

# Avoid: Hardcoded values
debug_mode = True  # Not configurable
```

### 3. Provide Sensible Defaults

Always provide reasonable defaults:

```python
# Good: Always has a fallback
build_type = env.get("BUILD_TYPE", "release")

# Avoid: Might fail if environment variable is missing
build_type = env.get("BUILD_TYPE")  # Could be None
```

### 4. Document Script Behavior

Include docstrings and comments:

```python
def generate_config(environment):
    """
    Generate build configuration for the specified environment.
    
    Args:
        environment: Target environment ("dev", "staging", "prod")
        
    Returns:
        Dictionary with build configuration
        
    Environment Variables:
        BUILD_TYPE: Override build type (debug/release)
        INSTALL_DIR: Override installation directory
    """
    # Implementation...
```

### 5. Handle Errors Gracefully

Check for required conditions and provide helpful error messages:

```python
def validate_environment():
    """Validate that required environment is set up."""
    
    # Check for required environment variables
    required_vars = ["PROJECT_NAME", "VERSION"]
    for var in required_vars:
        if not env.has(var):
            fail("Required environment variable {} is not set".format(var))
    
    # Check for required files
    if not path.exists("Cargo.toml"):
        fail("Cargo.toml not found. Are you in a Rust project directory?")

validate_environment()
```

## Advanced Use Cases

### Dynamic Dependency Selection

```python
# dynamic-deps.star
def select_dependencies():
    """Select optional dependencies based on target features."""
    
    features = []
    
    # Database support
    db_type = env.get("DATABASE", "none")
    if db_type == "postgres":
        features.append("postgres")
    elif db_type == "sqlite":
        features.append("sqlite")
    
    # Async runtime
    async_runtime = env.get("ASYNC_RUNTIME", "tokio")
    if async_runtime == "tokio":
        features.append("tokio-runtime")
    elif async_runtime == "async-std":
        features.append("async-std-runtime")
    
    # Serialization format
    if env.get("ENABLE_JSON", "true") == "true":
        features.append("json")
    if env.get("ENABLE_YAML", "false") == "true":
        features.append("yaml")
    
    return {"features": features}

select_dependencies()
```

### Multi-Target Builds

```python
# multi-target.star
def generate_multi_target_config():
    """Generate configuration for building multiple targets."""
    
    targets = env.get("BUILD_TARGETS", "x86_64-unknown-linux-gnu").split(",")
    
    configs = []
    for target in targets:
        target_config = {
            "project_path": ".",
            "build_type": "release",
            "install_dir": "./dist/" + target,
            "vendor": True,
            "cargo_flags": ["--target", target]
        }
        configs.append(target_config)
    
    return {"targets": configs}

generate_multi_target_config()
```

### Conditional Compilation

```python
# conditional-build.star
def should_build():
    """Determine if build should proceed based on conditions."""
    
    # Skip build on documentation-only changes
    changed_files = env.get("CHANGED_FILES", "").split()
    if all(f.endswith((".md", ".txt", ".rst")) for f in changed_files):
        return False
    
    # Skip build on weekends (for automated triggers)
    if time.weekday() in [5, 6] and env.get("SKIP_WEEKEND_BUILDS", "true") == "true":
        return False
    
    # Always build on release branches
    branch = env.get("BRANCH", "main")
    if branch.startswith("release/"):
        return True
    
    return True

if should_build():
    config = {
        "project_path": ".",
        "build_type": "release",
        "install_dir": "./install",
        "vendor": False
    }
else:
    config = {"skip_build": True}

config
```

This guide provides a comprehensive overview of using Starlark scripting with Black Anvil for advanced build automation and configuration management.