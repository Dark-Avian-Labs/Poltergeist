# Architecture Overview

Poltergeist follows a modular architecture organized as a Cargo workspace with clear separation of concerns. The codebase is divided into four main crates, each with specific responsibilities.

## Crate Structure

### `poltergeist-app` - Main Application
**Purpose**: Desktop UI application with Slint framework
**Key Responsibilities**:
- User interface and interaction
- Application state management
- System tray integration
- Internationalization (i18n)

**Source Files**:
- `/crates/poltergeist-app/src/main.rs` - Main application entry point
- `/crates/poltergeist-app/ui/main.slint` - UI definition (193K+ lines)
- `/crates/poltergeist-app/src/i18n.rs` - Internationalization support
- `/crates/poltergeist-app/src/picker.rs` - Snippet picker functionality
- `/crates/poltergeist-app/build.rs` - Build script with Windows resource compilation

### `poltergeist-core` - Business Logic
**Purpose**: Core data models, template engine, and business rules
**Key Responsibilities**:
- Data structures (Snippet, Folder, Node, etc.)
- Template token evaluation and matching
- Context-aware filtering logic
- Service contracts and interfaces

**Source Files**:
- `/crates/poltergeist-core/src/models.rs` - Primary data models
- `/crates/poltergeist-core/src/tokens.rs` - Token template engine
- `/crates/poltergeist-core/src/context.rs` - Application context management
- `/crates/poltergeist-core/src/contracts.rs` - Service contracts

### `poltergeist-io` - External Services
**Purpose**: File I/O, configuration, and external service integration
**Key Responsibilities**:
- Configuration loading and saving
- Team pack synchronization
- Database registry management
- DeepL translation service

**Source Files**:
- `/crates/poltergeist-io/src/config.rs` - Configuration management
- `/crates/poltergeist-io/src/team_pack.rs` - Team sharing functionality
- `/crates/poltergeist-io/src/database.rs` - Database CSV/XLSX parsing
- `/crates/poltergeist-io/src/translation.rs` - DeepL integration

### `poltergeist-platform-win` - Windows Integration
**Purpose**: Windows-specific platform code
**Key Responsibilities**:
- Text injection into applications
- Global hotkey management
- Window focus detection
- Single instance enforcement

**Source Files**:
- `/crates/poltergeist-platform-win/src/injector.rs` - Text injection system
- `/crates/poltergeist-platform-win/src/hotkeys.rs` - Global hotkey management
- `/crates/poltergeist-platform-win/src/focus.rs` - Window focus detection
- `/crates/poltergeist-platform-win/src/single_instance.rs` - Single instance logic
- `/crates/poltergeist-platform-win/src/ffi.rs` - Windows API FFI bindings

## Design Patterns

### Clean Separation
- Platform-specific code isolated in `poltergeist-platform-win`
- Business logic separated from UI in `poltergeist-core`
- External services and I/O isolated in `poltergeist-io`
- UI concerns contained in `poltergeist-app`

### Dependency Inversion
Core crate defines models (`Snippet`, `Folder`, `Node`) that are used by all other crates, preventing circular dependencies.

### Service-Oriented Architecture
IO crate provides services (configuration, translation, database) that can be consumed by other crates through defined interfaces.

### State Management
Central `AppState` struct in main application with extensive UI synchronization logic, ensuring consistent state across the application.

## Dependencies

### Core Dependencies
- **slint**: UI framework for declarative UI definitions
- **tokio**: Async runtime for non-blocking operations
- **tracing**: Structured logging for debugging
- **serde**: Serialization/deserialization for configuration
- **arboard**: Cross-platform clipboard access
- **global-hotkey**: Platform-agnostic hotkey management

### Windows-Specific
- **windows**: Windows API bindings via the `windows` crate
- FFI layer for direct Windows API calls when needed

### Service Dependencies
- **reqwest**: HTTP client for team sharing and translation
- **calamine**: Excel file parsing for database lookups
- **deepl**: DeepL translation API client

## Data Flow

1. **User Input** → Hotkey detected by `poltergeist-platform-win`
2. **Context Capture** → Active window context extracted via `poltergeist-platform-win`
3. **Snippet Filtering** → Context applied to filter snippets via `poltergeist-core`
4. **UI Display** → Filtered snippets shown in picker via `poltergeist-app`
5. **Selection & Injection** → User selects snippet, token evaluation via `poltergeist-core`, injection via `poltergeist-platform-win`

## Configuration Management

Configuration follows a portable approach:
- `poltergeist.json` lives next to executable
- No registry dependencies
- Edition detection via environment variable or `_admin.flag` file
- Team packs can be local or remote (UNC/HTTP)

## Platform Considerations

Currently Windows-only, but architecture allows for potential platform ports:
- Platform crate is isolated
- Core business logic is platform-agnostic
- UI uses Slint which supports multiple platforms

## Source Map

### Key Entry Points
- `/crates/poltergeist-app/src/main.rs` - Application entry
- `/crates/poltergeist-app/Cargo.toml` - App crate configuration
- `/Cargo.toml` - Workspace configuration

### Critical Interfaces
- `/crates/poltergeist-core/src/contracts.rs` - Service interfaces
- `/crates/poltergeist-platform-win/src/lib.rs` - Platform exports
- `/crates/poltergeist-io/src/lib.rs` - IO service exports

### Build System
- `/crates/poltergeist-app/build.rs` - Windows resource compilation
- `/.github/workflows/ci.yml` - CI/CD pipeline
- `/Cargo.lock` - Dependency versions

## Architectural Decisions

1. **Single Large UI File**: The `main.slint` file is 193K+ lines, containing the entire UI definition. This follows Slint's declarative approach but could be refactored for maintainability.

2. **Centralized FFI**: Windows API calls are centralized in `ffi.rs` for safety and maintainability, with recent refactoring moving from direct Windows crate calls to explicit FFI.

3. **Async Service Layer**: IO crate uses Tokio async runtime for non-blocking operations (HTTP requests, file I/O).

4. **Portable Configuration**: Configuration lives beside executable with no system-wide installation, supporting portable USB drives.

5. **Dual Edition Support**: Admin vs User editions with runtime detection via environment, flag file, or compile-time feature flag.

## Extension Points

1. **New Injection Modes**: Add to `InjectionMode` enum in `models.rs` and implement in `injector.rs`
2. **Additional Token Types**: Extend `Token` enum in `tokens.rs`
3. **New Database Formats**: Implement new parsers in `database.rs`
4. **Additional Platform Support**: Create new platform crate following `poltergeist-platform-win` patterns
5. **New Team Sync Protocols**: Extend `team_pack.rs` with additional sync strategies