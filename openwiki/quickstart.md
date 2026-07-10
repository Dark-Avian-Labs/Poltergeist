# Poltergeist OpenWiki Quickstart

Welcome to Poltergeist, a portable Windows snippet manager designed as a spiritual successor to GhostWriter and alternative to PhraseExpress.

## Repository Overview

Poltergeist is a Rust-based desktop application that provides intelligent snippet management with context-aware injection. Press a global hotkey (`Ctrl+Alt+Space` by default) to open a nested popup at your cursor, pick a snippet, and watch it get typed or pasted into the focused application field.

**Key Characteristics:**
- **Platform**: Windows 10/11 native application
- **Language**: Rust 1.77+
- **UI Framework**: Slint (declarative UI)
- **Architecture**: Modular crate workspace with clear separation of concerns
- **Target Users**: Power users, developers, support teams needing intelligent text expansion

## Quick Navigation

Start here for documentation:
- **[Architecture Overview](architecture/index.md)** – Understanding the crate structure and design patterns
- **[Core Concepts](core-concepts/index.md)** – Snippet models, token system, and business logic
- **[Windows Platform](platform/index.md)** – Windows-specific integrations and injection system
- **[Configuration & IO](configuration/index.md)** – Settings, team sharing, and external services
- **[Development Guide](development/index.md)** – Building, testing, and contributing
- **[User Workflows](workflows/index.md)** – How users interact with the application

## Repository Structure

```
Poltergeist/
├── Cargo.toml                 # Workspace configuration
├── crates/
│   ├── poltergeist-app/       # Main GUI application (Slint UI)
│   ├── poltergeist-core/      # Business logic and data models
│   ├── poltergeist-io/        # Configuration, team sharing, translation services
│   └── poltergeist-platform-win/  # Windows-specific platform code
├── assets/                    # Fonts, icons, and resources
├── openwiki/                  # This documentation
└── poltergeist.json          # Application configuration
```

## Core Features

1. **Context-Aware Snippets**: Snippets and folders can show/hide based on active window context using regex matching
2. **Rich Template Language**: Tokens for dates, clipboard content, waits, named keys, DeepL translation, database lookups, and conditional logic
3. **Multiple Injection Modes**: Clipboard (Ctrl+V), Clipboard (Shift+Insert), typing (Key Events), and typing (Web Terminal) for compatibility
4. **Team Collaboration**: Share snippets via UNC/local folders or HTTP(S) endpoints with automatic caching
5. **Per-Folder Hotkeys**: Assign hotkeys to top-level folders for direct submenu access
6. **Internationalization**: English, German, Spanish, and French UI translations
7. **Portable Runtime**: No installer or registry dependencies; configuration lives next to executable

## Getting Started

### From Source
```powershell
# Run from workspace root
cargo run -p poltergeist-app --bin poltergeist
```

### Requirements
- Windows 10/11
- Rust toolchain (rust-version = 1.77)
- Visual Studio Build Tools (C++ workload) if linker tools are missing

### Building Portable Executables
```powershell
# Standard user edition
cargo build -p poltergeist-app --release

# Fixed admin edition
cargo build -p poltergeist-app --release --features admin-edition
```

Output: `target/release/poltergeist.exe`

## Editions

Poltergeist supports two runtime editions resolved in this order:
1. `POLTERGEIST_EDITION=admin|user` environment variable
2. `_admin.flag` file beside the executable
3. Fallback: user edition

When built with `--features admin-edition`, runtime ignores env/flag and is always Admin.

## Key Source Files

- **Application Entry**: `/crates/poltergeist-app/src/main.rs` (main application logic)
- **UI Definition**: `/crates/poltergeist-app/ui/main.slint` (Slint UI definition)
- **Data Models**: `/crates/poltergeist-core/src/models.rs` (Snippet, Folder, Node, etc.)
- **Template Engine**: `/crates/poltergeist-core/src/tokens.rs` (token evaluation and matching)
- **Windows Injection**: `/crates/poltergeist-platform-win/src/injector.rs` (text injection system)
- **Configuration**: `/crates/poltergeist-io/src/config.rs` (settings loading/saving)
- **Translation Service**: `/crates/poltergeist-io/src/translation.rs` (DeepL integration)

## Next Steps

1. Read the **[TUTORIAL.md](../TUTORIAL.md)** for complete snippet syntax reference
2. Check **[Development Guide](development/index.md)** for contributor guidelines
3. Explore **[Architecture Overview](architecture/index.md)** to understand the codebase structure
4. Review **[Core Concepts](core-concepts/index.md)** to grasp the business logic