# Development Guide

This guide covers building, testing, contributing to, and maintaining the Poltergeist codebase.

## Development Environment Setup

### Prerequisites
- **Windows 10/11**: Primary development platform
- **Rust Toolchain**: Version 1.77 or higher
- **Visual Studio Build Tools**: C++ workload for linker tools
- **Git**: Version control

### Initial Setup
```powershell
# Clone the repository
git clone https://github.com/Dark-Avian-Labs/Poltergeist
cd Poltergeist

# Verify Rust installation
rustc --version  # Should be 1.77+

# Install required tools
cargo install cargo-watch  # For development watching
cargo install cargo-tarpaulin  # For code coverage
```

## Building from Source

### Development Build
```powershell
# Build all crates in debug mode
cargo build

# Build specific crate
cargo build -p poltergeist-app

# Build with watch mode (auto-rebuild on changes)
cargo watch -x "build -p poltergeist-app"
```

### Release Builds
```powershell
# Standard user edition
cargo build -p poltergeist-app --release

# Admin edition (feature-flagged)
cargo build -p poltergeist-app --release --features admin-edition

# Check binary size
ls -la target/release/poltergeist.exe
```

### Build Artifacts
- `target/debug/poltergeist.exe` - Debug binary with symbols
- `target/release/poltergeist.exe` - Optimized release binary
- `target/` directories contain intermediate build artifacts

## Running from Source

### Development Execution
```powershell
# Run from workspace root
cargo run -p poltergeist-app --bin poltergeist

# Run with specific features
cargo run -p poltergeist-app --bin poltergeist --features admin-edition

# Run with environment variables
$env:POLTERGEIST_EDITION="admin"; cargo run -p poltergeist-app
```

### Debug Execution
```powershell
# Run with debug logging
$env:RUST_LOG="debug"; cargo run -p poltergeist-app

# Run with specific module logging
$env:RUST_LOG="poltergeist_core=debug,poltergeist_platform_win=info"; cargo run
```

## Testing

### Test Suite
```powershell
# Run all tests
cargo test --workspace

# Run tests for specific crate
cargo test -p poltergeist-core
cargo test -p poltergeist-io
cargo test -p poltergeist-platform-win

# Run with test output
cargo test --workspace -- --nocapture

# Run integration tests
cargo test --test "*" --workspace
```

### Test Coverage
```powershell
# Generate coverage report (requires cargo-tarpaulin)
cargo tarpaulin --workspace --out Html

# Coverage for specific crate
cargo tarpaulin -p poltergeist-core --out Html
```

### Test Categories
1. **Unit Tests**: Individual function tests within each module
2. **Integration Tests**: Crate-to-crate interface tests
3. **Platform Tests**: Windows-specific functionality tests
4. **UI Tests**: Slint component tests (if applicable)

## Code Quality Checks

### Formatting
```powershell
# Format all code
cargo fmt --all

# Check formatting without applying
cargo fmt --all -- --check
```

### Linting
```powershell
# Run clippy on all crates and targets
cargo clippy --workspace --all-targets -- -D warnings

# Specific crate linting
cargo clippy -p poltergeist-app -- -D warnings

# Allow specific lints where needed
# #[allow(clippy::too_many_arguments)]
```

### Static Analysis
```powershell
# Security audit
cargo audit

# Dependency updates
cargo outdated

# Dead code detection
cargo check --workspace
```

## Development Workflow

### Branch Strategy
1. **main**: Production-ready code
2. **feature/**: New features and enhancements
3. **bugfix/**: Bug fixes and patches
4. **release/**: Release preparation branches

### Commit Guidelines
- Use conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`, `chore:`
- Keep commits focused and atomic
- Reference issue numbers: `fix: hotkey registration #123`

### Pull Request Process
1. Create feature/bugfix branch from `main`
2. Implement changes with tests
3. Run quality checks: `cargo fmt`, `cargo clippy`, `cargo test`
4. Create PR with description of changes
5. Address review feedback
6. Merge after CI passes and approvals

## CI/CD Pipeline

### GitHub Actions
The repository uses GitHub Actions for continuous integration:

**Workflows**:
- `/.github/workflows/ci.yml` - Main CI pipeline
- `/.github/workflows/pr.yml` - PR validation

**CI Steps**:
1. **Setup**: Install Rust, cache dependencies
2. **Check**: `cargo check --workspace`
3. **Format**: `cargo fmt --all -- --check`
4. **Lint**: `cargo clippy --workspace --all-targets -- -D warnings`
5. **Test**: `cargo test --workspace`
6. **Build**: `cargo build --release`
7. **Artifacts**: Package release binaries

### Nightly Builds
CI publishes two Windows zip artifacts:
1. **User Edition**: Standard portable build
2. **Admin Edition**: Fixed admin edition build

### Release Process
1. Update version in `Cargo.toml` files
2. Update changelog/documentation
3. Create release tag: `git tag v1.2.3`
4. Push tag: `git push origin v1.2.3`
5. CI automatically builds and uploads release artifacts

## Debugging

### Common Issues

#### Build Issues
```powershell
# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check toolchain
rustup update
```

#### Runtime Issues
```powershell
# Enable debug logging
$env:RUST_LOG="debug"

# Check Windows Event Viewer for crashes
# Application logs: %APPDATA%\Poltergeist\logs\
```

#### Hotkey Issues
- Check for hotkey conflicts with other applications
- Verify running with appropriate privileges (admin vs user)
- Check Windows hotkey registration limits

### Debug Tools
1. **Rust Debugger**: `rust-gdb` or Visual Studio Code with Rust extension
2. **Process Monitor**: Sysinternals ProcMon for Windows API calls
3. **Window Spy**: Inspect window titles and classes
4. **Input Tracing**: Debug injection issues

## Performance Profiling

### Build Profiling
```powershell
# Profile build times
cargo build --timings

# Identify slow dependencies
cargo tree
```

### Runtime Profiling
```powershell
# Profile with perf (Linux) or Windows Performance Toolkit
# Memory profiling with valgrind or DHAT
```

### Performance Considerations
1. **Startup Time**: Lazy loading of databases and team packs
2. **Memory Usage**: Cache management and cleanup
3. **Injection Speed**: Token evaluation and text injection optimization
4. **UI Responsiveness**: Async operations and background processing

## Internationalization (i18n)

### Translation Files
- Location: `/crates/poltergeist-app/lang/`
- Format: Gettext `.po` files
- Languages: English (en), German (de), Spanish (es), French (fr)

### Adding New Language
1. Create directory: `lang/{code}/LC_MESSAGES/`
2. Copy `poltergeist-app.po` template
3. Translate strings using `_annotate_tr.py` scripts
4. Update `i18n.rs` to include new language
5. Test with `$env:LANG="{code}"`

### Translation Tools
```powershell
# Generate/update PO files
python lang/_generate_po.py

# Annotate translations
python lang/_annotate_tr.py
python lang/_annotate_status_tr.py
```

## UI Development

### Slint Framework
Poltergeist uses Slint for declarative UI:

**UI File**: `/crates/poltergeist-app/ui/main.slint`
**Build Integration**: Slint compiler invoked via build.rs

### UI Development Tips
1. **Hot Reload**: `cargo watch` for Slint file changes
2. **Component Testing**: Isolate components in separate `.slint` files
3. **State Management**: Use Slint properties and callbacks
4. **Internationalization**: Use `tr!()` macro for translatable strings

### UI Debugging
```rust
// Enable Slint debug logging
slint::set_logging_enabled(true);

// Use Slint inspector
// Add `--slint-inspector` flag when running
```

## Platform-Specific Development

### Windows Development
```powershell
# Windows SDK requirements
# Visual Studio Build Tools with C++ workload

# Common Windows development issues:
# - Linker errors: Install Windows SDK
# - DLL dependencies: Check PATH includes VC++ redistributables
# - UAC issues: Run as administrator when needed
```

### Cross-Platform Considerations
While currently Windows-only, the architecture supports potential ports:
- Platform crate isolated from business logic
- Core contracts define platform-agnostic interfaces
- UI uses Slint which supports multiple platforms

## Dependency Management

### Cargo Workspace
```toml
# Root Cargo.toml defines workspace
[workspace]
members = [
    "crates/poltergeist-app",
    "crates/poltergeist-core", 
    "crates/poltergeist-io",
    "crates/poltergeist-platform-win",
]
```

### Dependency Updates
```powershell
# Check for updates
cargo outdated

# Update dependencies
cargo update

# Security audit
cargo audit

# Update specific crate
cargo update -p serde
```

### Version Pinning
- Critical dependencies may be pinned for stability
- Regular updates recommended for security patches
- Test thoroughly after dependency updates

## Documentation

### Code Documentation
```rust
/// Documentation comments for public API
/// 
/// # Examples
/// ```
/// use poltergeist_core::models::Snippet;
/// let snippet = Snippet::new("Example");
/// ```
pub struct Snippet {
    // ...
}
```

### Building Documentation
```powershell
# Generate API documentation
cargo doc --workspace --no-deps

# Open documentation in browser
cargo doc --open
```

### Documentation Standards
1. **Public API**: Comprehensive documentation with examples
2. **Complex Algorithms**: Explanation of logic and edge cases
3. **Safety Notes**: `unsafe` blocks must be documented
4. **Performance Considerations**: Note expensive operations

## Contributing Guidelines

### Code Style
- Follow Rustfmt configuration
- Adhere to Clippy warnings
- Use meaningful variable names
- Comment complex logic

### Testing Requirements
- New features require tests
- Bug fixes include regression tests
- Integration tests for cross-crate functionality
- Platform tests for Windows-specific code

### Review Process
1. **Code Review**: At least one reviewer required
2. **CI Checks**: All tests must pass
3. **Documentation**: Update relevant documentation
4. **Backward Compatibility**: Consider impact on existing users

### Issue Reporting
1. **Bug Reports**: Include steps to reproduce, expected vs actual behavior
2. **Feature Requests**: Describe use case and proposed solution
3. **Security Issues**: Report via secure channel

## Maintenance Tasks

### Regular Maintenance
```powershell
# Weekly tasks
cargo update
cargo audit
cargo clippy --workspace --all-targets -- -D warnings

# Monthly tasks  
Update Rust toolchain
Review dependency licenses
Performance profiling
```

### Release Preparation
1. Update version numbers
2. Update changelog
3. Run full test suite
4. Performance regression testing
5. Documentation review

### Deprecation Process
1. Mark deprecated items with `#[deprecated]`
2. Provide migration path in documentation
3. Remove in next major version

## Getting Help

### Resources
- **Repository Issues**: GitHub issue tracker
- **Documentation**: This OpenWiki and README/TUTORIAL
- **Code Examples**: Existing implementation patterns
- **Community**: GitHub discussions (if enabled)

### Common Troubleshooting
- Check `.github/workflows/ci.yml` for CI configuration
- Review recent commits for similar issues
- Search existing issues for solutions
- Ask in PR reviews for guidance