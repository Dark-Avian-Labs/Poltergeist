# Windows Platform Integration

This section covers the Windows-specific platform code that enables Poltergeist's core functionality: hotkey detection, text injection, focus management, and single instance enforcement.

## Platform Architecture

### Crate Overview (`poltergeist-platform-win`)
**Purpose**: Windows-specific implementations of platform-agnostic interfaces defined in core contracts.

**Key Modules**:
- `injector.rs` - Text injection into applications
- `hotkeys.rs` - Global hotkey management  
- `focus.rs` - Window focus detection and context capture
- `single_instance.rs` - Single instance enforcement
- `ffi.rs` - Windows API FFI bindings
- `cursor.rs` - Mouse cursor positioning
- `theme.rs` - System theme detection

**Source**: `/crates/poltergeist-platform-win/src/lib.rs` - Module declarations

## Text Injection System

### Injection Modes (`injector.rs`)
Poltergeist supports four injection modes, each optimized for different scenarios:

#### 1. ClipboardCtrlV
```rust
// Standard clipboard injection
1. Save current clipboard content
2. Copy snippet text to clipboard
3. Send Ctrl+V keystroke
4. Restore original clipboard
```
**Use Case**: Most applications, reliable cross-platform compatibility

#### 2. ClipboardShiftInsert
```rust
// Terminal-compatible clipboard injection  
1. Save current clipboard content
2. Copy snippet text to clipboard
3. Send Shift+Insert keystroke
4. Restore original clipboard
```
**Use Case**: Terminal applications, legacy systems

#### 3. TypingKeyEvents
```rust
// Real key event simulation
1. For each character in snippet:
   - Generate key down/up events
   - Respect keyboard layout
   - Handle special characters
```
**Use Case**: Applications that block clipboard paste, security-sensitive fields

#### 4. TypingWebTerminal
```rust
// Win32 SendInput with scan codes
1. Use VK (virtual key) codes
2. Include scan codes for key identification
3. Segment long text to avoid input queue overflow
```
**Use Case**: Web terminals, applications with custom key handling

**Source**: `/crates/poltergeist-platform-win/src/injector.rs` - `TextInjector` struct and `InjectionMode` implementations

### Injection Process
1. **Token Evaluation**: Snippet body evaluated to final text via `TokenEngine`
2. **Mode Selection**: Appropriate injection method based on snippet configuration
3. **Focus Verification**: Ensure target window still has focus
4. **Injection Execution**: Text injected using selected mode
5. **Error Recovery**: Fallback strategies if injection fails

## Global Hotkey Management

### Hotkey Registration (`hotkeys.rs`)
```rust
// Hotkey configuration
Hotkey {
    modifiers: Modifiers::CONTROL | Modifiers::ALT,
    key_code: KeyCode::Space,
}
```

**Features**:
- Global system-wide hotkey registration
- Modifier support (Ctrl, Alt, Shift, Win)
- Hotkey conflict detection
- Dynamic hotkey changes at runtime

**Implementation**:
- Uses `global-hotkey` crate for cross-platform abstraction
- Windows-specific optimizations via raw Win32 API when needed
- Hotkey callbacks routed to main application

**Source**: `/crates/poltergeist-platform-win/src/hotkeys.rs` - `HotkeyManager` struct

## Window Focus Detection

### Context Capture (`focus.rs`)
When the hotkey is pressed, Poltergeist captures:

1. **Active Window**:
   - Window title
   - Process name (executable)
   - Window class name
   - Process ID

2. **Selected Text**:
   - Currently selected text in active window
   - Used for context variable extraction

3. **Cursor Position**:
   - Mouse coordinates for popup placement
   - Screen-relative positioning

**Source**: `/crates/poltergeist-platform-win/src/focus.rs` - `FocusDetector` struct

### Focus Management
- **Focus Tracking**: Monitor focus changes during injection
- **Focus Restoration**: Return focus to original window after popup
- **Focus Validation**: Verify target window still focused before injection

## Single Instance Enforcement

### Instance Guard (`single_instance.rs`)
Prevents multiple instances of Poltergeist from running simultaneously.

**Implementation Strategies**:

#### 1. Named Mutex (User Edition)
```rust
// Create a named mutex visible to user session
CreateMutexW(NULL, FALSE, L"Local\\Poltergeist-User");
```

#### 2. Global Mutex (Admin Edition)  
```rust
// Create a global mutex visible across sessions
CreateMutexW(NULL, FALSE, L"Global\\Poltergeist-Admin");
```

**Features**:
- Different mutex scopes for admin vs user editions
- Automatic instance detection and notification
- Graceful handling of terminated instances

**Source**: `/crates/poltergeist-platform-win/src/single_instance.rs` - `InstanceGuard` struct

## Windows API FFI Layer

### FFI Architecture (`ffi.rs`)
Recent refactoring moved from direct `windows` crate calls to explicit FFI for:

1. **Safety**: Explicit control over unsafe Windows API calls
2. **Performance**: Reduced overhead for frequent operations
3. **Maintainability**: Centralized Windows API usage

**Key FFI Functions**:
- `GetForegroundWindow` - Active window detection
- `GetWindowTextW` - Window title retrieval
- `GetClassNameW` - Window class name
- `GetCursorPos` - Mouse cursor position
- `SetCursorPos` - Cursor positioning
- `SendInput` - Keystroke injection

**Source**: `/crates/poltergeist-platform-win/src/ffi.rs` - FFI function declarations

### FFI vs Windows Crate
```rust
// Before: Using windows crate
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

// After: Explicit FFI
extern "system" {
    fn GetForegroundWindow() -> HWND;
}
```

**Benefits**:
- No dependency on windows crate for simple calls
- Fine-grained control over calling conventions
- Potential performance improvements

## System Integration

### Cursor Management (`cursor.rs`)
- Popup positioning relative to cursor
- Cursor confinement during popup display
- Multi-monitor coordinate translation

### Theme Detection (`theme.rs`)
- System dark/light mode detection
- UI theme adaptation
- High contrast mode support

### Notification System
- Windows toast notifications
- Balloon tips for system tray
- Sound feedback for actions

## Security Considerations

### Injection Security
1. **User Consent**: Only inject into user-approved applications
2. **Focus Verification**: Prevent injection into unexpected windows
3. **Rate Limiting**: Throttle injection to prevent abuse
4. **Input Validation**: Sanitize snippet content before injection

### Instance Security
1. **Session Isolation**: Admin vs user instance separation
2. **Mutex Permissions**: Appropriate access control for named objects
3. **Cleanup**: Proper release of system resources on exit

## Platform-Specific Optimizations

### Performance Optimizations
1. **Input Queue Management**: Segment long text to avoid queue overflow
2. **Clipboard Caching**: Reduce clipboard operations for repeated snippets
3. **Focus Cache**: Cache window information to reduce API calls
4. **Hotkey Optimization**: Use low-level hooks for responsive hotkey detection

### Compatibility Features
1. **DPI Awareness**: Proper scaling across different DPI settings
2. **UAC Compatibility**: Admin vs user privilege handling
3. **Terminal Compatibility**: Special handling for console applications
4. **Remote Desktop**: Support for RDP sessions

## Error Handling

### Injection Errors
- **Focus Lost**: Target window no longer active
- **Access Denied**: Insufficient permissions for injection
- **Queue Full**: Input queue overflow
- **Clipboard Error**: Clipboard access failure

### Recovery Strategies
1. **Retry Logic**: Automatic retry with backoff
2. **Fallback Modes**: Try alternative injection methods
3. **User Notification**: Inform user of injection failure
4. **Logging**: Detailed error logging for debugging

## Testing Considerations

### Unit Testing
- Mock Windows API for platform-independent tests
- Focus on business logic rather than Win32 integration

### Integration Testing
- Test with real Windows applications
- Verify injection across different window types
- Test hotkey registration and conflict resolution

### Manual Testing Matrix
- Windows 10 vs Windows 11
- Different DPI settings
- Admin vs user sessions
- Multiple monitor configurations
- Remote Desktop sessions

## Extension Points

### New Injection Modes
1. Add to `InjectionMode` enum in core models
2. Implement in `TextInjector` in platform crate
3. Add UI configuration in main application

### Additional System Integration
1. **Clipboard History**: Enhanced clipboard management
2. **Window Snap**: Integration with Windows Snap features
3. **Voice Input**: Voice-activated snippet insertion
4. **Gesture Support**: Mouse gesture recognition

### Cross-Platform Considerations
While currently Windows-only, the architecture supports potential ports:
- Platform crate is isolated from business logic
- Core contracts define platform-agnostic interfaces
- UI uses Slint which supports multiple platforms