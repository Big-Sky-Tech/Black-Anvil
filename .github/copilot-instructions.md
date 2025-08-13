# Black Anvil

Black Anvil is an experimental Rust-based installer that compiles and installs Rust projects. It provides a simple interactive command-line interface to choose build options and installation path, then invokes `cargo build` and copies the resulting binary to the selected location.

**ALWAYS follow these instructions first. Only search or use bash commands when you encounter unexpected information that does not match these instructions or when the instructions are incomplete or found to be in error.**

## Working Effectively

### Bootstrap and Build
- `cargo build` -- takes 5-7 seconds after first build. NEVER CANCEL. Set timeout to 60+ minutes for initial builds.
- `cargo build --release` -- takes 10-15 seconds after first build. NEVER CANCEL. Set timeout to 60+ minutes for initial builds.
- `cargo clean` -- takes <1 second. Use to force complete rebuild.
- `cargo check` -- takes 5-8 seconds. Faster than full build for syntax checking.

### Initial Build Timing (clean slate)
- **First `cargo build`**: 35+ seconds. NEVER CANCEL. Set timeout to 60+ minutes.
- **First `cargo build --release`**: 15+ seconds. NEVER CANCEL. Set timeout to 60+ minutes.
- **`cargo test`**: 1-2 seconds (but no tests exist currently). NEVER CANCEL. Set timeout to 30+ minutes.

### Running the Application
- **Interactive mode**: `./target/debug/black-anvil` (no arguments)
- **Config file mode**: `./target/debug/black-anvil path/to/config.toml`
- The application will prompt for:
  - Project path (default: ".")
  - Build type (release/debug, default: release)  
  - Installation directory (default: "./install")
  - Whether to vendor dependencies (default: no)

### Example Config File Format
```toml
project_path = "."
build_type = "release"
install_dir = "./install"
vendor = false
```

## Validation

- **ALWAYS** run through at least one complete end-to-end scenario after making changes.
- **Test scenarios to validate**:
  1. **Self-compilation**: Use Black Anvil to install itself using config file mode
  2. **External project**: Create a simple Rust project and install it using Black Anvil
  3. **Config file mode**: Test with a TOML config file (recommended over interactive mode)
  4. **Vendor dependencies**: Test with `vendor = true` in config
- **Format and lint**: Always run `cargo fmt` and `cargo clippy --fix --allow-dirty` before you are done or the CI (.github/workflows/rust-build-and-test.yml) will fail.
- **Build validation**: Always test both debug and release builds work: `cargo build && cargo build --release`

## Common Tasks

### Linting and Formatting
- `cargo fmt` -- applies Rust formatting (< 1 second)
- `cargo fmt --check` -- checks if formatting is needed
- `cargo clippy` -- runs linter (~5 seconds)
- `cargo clippy --fix --allow-dirty` -- applies automatic fixes

### Development Workflow
1. Make your changes
2. Run `cargo check` to verify syntax
3. Run `cargo build` to compile
4. Test the application manually (see validation scenarios above)
5. Run `cargo fmt && cargo clippy --fix --allow-dirty`
6. Commit your changes

### Testing the Application
```bash
# Test with config file (RECOMMENDED)
cat > test-config.toml << EOF
project_path = "."
build_type = "debug"
install_dir = "./test_install"
vendor = false
EOF
./target/debug/black-anvil test-config.toml

# Test self-installation (config file approach)
cat > self-install.toml << EOF
project_path = "."
build_type = "release"
install_dir = "./self_install"
vendor = false
EOF
./target/debug/black-anvil self-install.toml

# Interactive mode (manual testing only - does not work well with scripted input)
./target/debug/black-anvil
```

## Project Structure

### Key Files
- `src/main.rs` -- Entry point with CLI argument parsing and interactive prompts
- `src/lib.rs` -- Core functions: `build_project()`, `copy_binary()`, `vendor_dependencies()`
- `src/config.rs` -- TOML configuration file parsing
- `Cargo.toml` -- Dependencies: anyhow, dialoguer, serde, toml
- `.github/workflows/rust-build-and-test.yml` -- CI pipeline (build + test)

### Dependencies
- **anyhow**: Error handling with context
- **dialoguer**: Interactive CLI prompts (Select, Input, Confirm)
- **serde + toml**: Configuration file parsing
- Standard Rust toolchain (rustc 1.88.0, cargo 1.88.0)

### Core Functionality
1. **build_project()**: Runs `cargo build` (with `--release` flag for release builds)
2. **copy_binary()**: Parses Cargo.toml to get binary name, copies from `target/{build_type}/` to install directory
3. **vendor_dependencies()**: Runs `cargo vendor`, copies vendor directory to install location

## Troubleshooting

### Common Issues
- **"Failed to read config file --help"**: The application doesn't support --help flag. Run without arguments for interactive mode.
- **Build failures**: Ensure you're in a valid Rust project directory with Cargo.toml
- **Permission errors**: Ensure write access to installation directory
- **Vendor directory not found**: cargo vendor only creates vendor dir if there are external dependencies
- **"Text file busy" when installing**: Cannot overwrite a running binary. Use a different install directory or stop the process first.
- **Interactive mode with scripts**: Interactive prompts don't work well with automated input. Use config files for scripted testing.

### Build Times
- **Clean builds**: 30+ seconds (debug), 15+ seconds (release)
- **Incremental builds**: 5-7 seconds typically
- **If builds seem stuck**: Wait at least 60 minutes before considering cancellation

### File Structure After Installation
```
install_directory/
├── binary_name          # The compiled binary
└── vendor/              # Only if vendor=true
    ├── crate1/
    ├── crate2/
    └── ...
```

## CI Pipeline
The `.github/workflows/rust-build-and-test.yml` workflow runs:
1. `cargo build --verbose`
2. `cargo test --verbose`

Ensure your changes pass both commands locally before committing.