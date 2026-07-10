# Core Concepts

This section covers the fundamental business logic and data models that power Poltergeist's intelligent snippet management system.

## Data Models

### Snippet (`models.rs`)
The core unit of content that can be injected into applications.

**Key Properties**:
- `name`: Display name shown in picker
- `body`: Template text with tokens to be evaluated
- `injection_mode`: How the snippet is injected (Clipboard, Typing, etc.)
- `filters`: Conditions for when the snippet should be shown
- `hotkey`: Optional direct activation hotkey
- `icon`: Optional icon identifier

**Source**: `/crates/poltergeist-core/src/models.rs` - `Snippet` struct

### Folder (`models.rs`)
Container for organizing snippets and subfolders hierarchically.

**Key Properties**:
- `name`: Display name
- `children`: Nested snippets and folders
- `filters`: Conditions for when the folder should be shown
- `hotkey`: Optional direct activation hotkey
- `icon`: Optional icon identifier

**Source**: `/crates/poltergeist-core/src/models.rs` - `Folder` struct

### Node (`models.rs`)
Union type representing either a `Snippet` or `Folder`, enabling recursive tree structures.

**Source**: `/crates/poltergeist-core/src/models.rs` - `Node` enum

### Injection Modes (`models.rs`)
Different methods for injecting text into target applications:

1. **ClipboardCtrlV**: Copy to clipboard, send Ctrl+V, restore original clipboard
2. **ClipboardShiftInsert**: Copy to clipboard, send Shift+Insert, restore clipboard
3. **TypingKeyEvents**: Simulate real key events (character-by-character typing)
4. **TypingWebTerminal**: Win32 `SendInput` with VK + scan codes for web terminals

**Source**: `/crates/poltergeist-core/src/models.rs` - `InjectionMode` enum

## Template Token System

### Token Types (`tokens.rs`)
The template engine supports a rich set of tokens for dynamic content:

1. **Text Tokens**: Plain text segments
2. **Variable Tokens**: `{VAR=name}` - Context variables from regex captures
3. **Date/Time Tokens**: `{DATE}`, `{TIME}`, `{DATETIME}` with format options
4. **Clipboard Tokens**: `{CLIPBOARD}` - Current clipboard content
5. **Wait Tokens**: `{WAIT=ms}` - Pause execution for milliseconds
6. **Key Tokens**: `{KEY=name}` - Simulate key presses (Enter, Tab, etc.)
7. **Database Tokens**: `{DATABASE=name}` - Lookup values from CSV/XLSX databases
8. **Include Tokens**: `{INCLUDE=id}` - Include other snippets by ID
9. **Translation Tokens**: `{DEEPL=text}` - Translate text via DeepL service
10. **Conditional Tokens**: `{IF}`, `{ELSIF}`, `{ELSE}`, `{END}` - Flow control

**Source**: `/crates/poltergeist-core/src/tokens.rs` - `Token` enum

### Token Evaluation (`tokens.rs`)
The `TokenEngine` evaluates tokens in sequence:
1. Parses template string into token stream
2. Evaluates each token with current context
3. Handles variable substitution, conditionals, and includes
4. Returns final text for injection

**Source**: `/crates/poltergeist-core/src/tokens.rs` - `TokenEngine` struct

## Context-Aware Filtering

### Context Extraction
Poltergeist can extract context from the active window using regex patterns:

```regex
# Example: Extract ticket number and component
(?P<ticket>[A-Z]+-\d+)-(?P<component>[A-Za-z]+)
```

Named capture groups become variables available to:
- Snippet/folder `Show when...` filters
- `{VAR=name}` tokens in snippet bodies
- `{DATABASE=...}` lookups

**Source**: `/crates/poltergeist-core/src/context.rs` - `ContextExtraction` struct

### Filter Rules (`models.rs`)
Filters define when snippets/folders should be visible based on context:

```rust
// Example filter structure
Filter {
    field: FilterField::WindowTitle,  // What to match against
    operator: FilterOperator::Contains,  // How to match
    value: "Visual Studio Code",  // Value to match
    case_sensitive: false,
}
```

**Filter Fields**:
- `WindowTitle`: Active window title
- `ProcessName`: Executable name
- `ClassName`: Window class name
- `Variable`: Context variable from regex capture

**Filter Operators**:
- `Contains`, `NotContains`
- `Equals`, `NotEquals`
- `StartsWith`, `EndsWith`
- `MatchesRegex`, `NotMatchesRegex`
- `IsEmpty`, `IsNotEmpty`

**Source**: `/crates/poltergeist-core/src/models.rs` - `Filter` struct and related enums

## Business Logic

### Snippet Matching (`tokens.rs`)
The `matches_context` method evaluates whether a snippet/folder should be shown based on:
1. Current application context (window title, process name, etc.)
2. Extracted context variables
3. Filter rules defined on the snippet/folder

**Source**: `/crates/poltergeist-core/src/tokens.rs` - `matches_context` method

### Tree Navigation
The snippet hierarchy uses a tree structure:
- Root contains personal and team sections
- Unlimited nesting of folders
- Drag-and-drop reordering support
- Hotkeys can be assigned to any top-level folder

## Service Contracts

### Contracts Interface (`contracts.rs`)
Defines interfaces for services that can be implemented by different crates:

```rust
pub trait DatabaseProvider {
    fn lookup(&self, database_name: &str, key: &str) -> Option<String>;
}

pub trait TranslationProvider {
    fn translate(&self, text: &str, target_lang: &str) -> Result<String, TranslationError>;
}
```

**Source**: `/crates/poltergeist-core/src/contracts.rs` - Service trait definitions

## Configuration Models

### Settings (`models.rs`)
Application-wide settings including:
- Global hotkey configuration
- Context extraction regex patterns
- UI preferences (theme, language, etc.)
- Team pack configuration
- Database registry

**Source**: `/crates/poltergeist-core/src/models.rs` - `Settings` struct

### Team Configuration
- **Local Team Packs**: UNC paths or local folders
- **Remote Team Packs**: HTTP(S) endpoints with caching
- **Cache Strategy**: Automatic fallback when remote is unavailable
- **Sync Interval**: Configurable synchronization frequency

## Key Algorithms

### Token Parsing
1. Scan template string for `{` and `}` delimiters
2. Parse token type and parameters
3. Build token stream with proper nesting for conditionals
4. Validate token syntax

### Context Evaluation
1. Capture active window information
2. Apply regex patterns to extract variables
3. Match extracted context against filter rules
4. Return filtered tree of visible snippets/folders

### Database Lookup
1. Parse CSV/XLSX files into memory-mapped structures
2. Index by key column for fast lookups
3. Support multiple value columns with dot notation: `{DATABASE=employees.name}`

## Domain-Specific Language

### Template Syntax Examples
```
# Simple variable substitution
Hello {VAR=name}, your ticket is {VAR=ticket}

# Conditional logic
{IF VAR=priority == "high"}
URGENT: {VAR=description}
{ELSIF VAR=priority == "medium"}
Priority: {VAR=description}
{ELSE}
{VAR=description}
{END}

# Database lookup
Contact: {DATABASE=employees.{VAR=assigned_to}.email}

# Translation
{DEEPL=Please review this document}=de
```

### Filter Syntax Examples
```
# Show only in Visual Studio Code
WindowTitle Contains "Visual Studio Code"

# Hide in specific process
ProcessName NotEquals "notepad.exe"

# Context-dependent visibility
Variable component Equals "backend"
```

## Performance Considerations

1. **Token Caching**: Frequently used tokens (date/time) are cached
2. **Database Indexing**: CSV/XLSX files are indexed for O(1) lookups
3. **Tree Filtering**: Context filtering happens once per hotkey press
4. **Async Operations**: Translation and team sync are non-blocking

## Extension Points

1. **New Token Types**: Add variants to `Token` enum in `tokens.rs`
2. **Additional Filter Fields**: Extend `FilterField` enum in `models.rs`
3. **Custom Database Formats**: Implement new `DatabaseProvider` implementations
4. **Advanced Conditional Logic**: Enhance `TokenEngine` evaluation rules