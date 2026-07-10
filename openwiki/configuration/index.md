# Configuration & IO Services

This section covers Poltergeist's configuration management, team collaboration features, database integration, and external service connections.

## Configuration Management

### Config File Structure (`poltergeist.json`)
The main configuration file lives next to the executable for portable operation.

**Location**: `poltergeist.json` (beside `poltergeist.exe`)

**Structure**:
```json
{
  "version": "1.0",
  "edition": "user",
  "settings": {
    "hotkey": "Ctrl+Alt+Space",
    "language": "en",
    "theme": "system",
    "context_extraction": [
      {
        "name": "ticket_pattern",
        "regex": "(?P<ticket>[A-Z]+-\\d+)"
      }
    ]
  },
  "snippets": {
    "personal": [],
    "team": []
  },
  "databases": {
    "employees": "data/employees.csv"
  },
  "team_packs": [
    {
      "name": "Engineering",
      "type": "http",
      "url": "https://team.example.com/snippets.json",
      "cache_ttl": 300
    }
  ]
}
```

**Source**: `/crates/poltergeist-io/src/config.rs` - `Config` struct

### Edition Detection
Poltergeist supports two editions with detection order:

1. **Environment Variable**: `POLTERGEIST_EDITION=admin|user`
2. **Flag File**: `_admin.flag` file beside executable
3. **Fallback**: User edition

**Admin Edition Features**:
- Global mutex for system-wide single instance
- Elevated privileges for certain operations
- Different configuration paths

**Source**: `/crates/poltergeist-io/src/config.rs` - edition detection logic

### Config Loading (`config.rs`)
```rust
// Configuration loading sequence
1. Check for config file next to executable
2. Load and parse JSON with schema validation
3. Apply environment overrides
4. Merge with default settings
5. Validate hotkey and other settings
```

**Error Handling**:
- Graceful fallback to defaults if config corrupted
- Automatic backup of corrupted config files
- Migration utilities for config version upgrades

## Team Collaboration

### Team Pack System (`team_pack.rs`)
Team packs enable snippet sharing across teams with automatic synchronization.

**Pack Types**:

#### 1. Local/UNC Packs
```rust
LocalTeamPack {
    path: "\\\\server\\share\\snippets",
    cache_dir: "cache\\team",
}
```
- Direct file system access
- Suitable for network shares
- No authentication required

#### 2. HTTP(S) Packs
```rust
HttpTeamPack {
    url: "https://api.example.com/snippets",
    cache_ttl: 300, // 5 minutes
    auth_token: Optional<String>,
}
```
- Remote synchronization
- Cache with TTL for offline use
- Optional authentication support

**Source**: `/crates/poltergeist-io/src/team_pack.rs` - `TeamPack` enum and implementations

### Synchronization Process
```rust
// Team pack synchronization
1. Check cache validity (based on TTL)
2. If cache stale or missing:
   a. Fetch from remote source
   b. Parse and validate snippet tree
   c. Update local cache
3. Merge with personal snippets
4. Apply context filtering
```

**Cache Strategy**:
- **Read-Through Cache**: Attempt remote, fallback to cache
- **TTL-Based Invalidation**: Configurable cache lifetime
- **Delta Updates**: Only fetch changes when supported
- **Conflict Resolution**: Last-write-wins for overlapping snippets

## Database Integration

### Database Registry (`database.rs`)
CSV and Excel (XLSX) files can be registered as lookup databases for snippet tokens.

**Supported Formats**:
1. **CSV**: Comma-separated values with header row
2. **XLSX**: Microsoft Excel files (via calamine library)
3. **JSON**: Planned future support

**Registration**:
```json
{
  "databases": {
    "employees": "data/employees.csv",
    "products": "shared/products.xlsx"
  }
}
```

**Source**: `/crates/poltergeist-io/src/database.rs` - `DatabaseRegistry` struct

### Lookup Syntax
```
# Basic lookup by key
{DATABASE=employees.john.doe}

# With variable substitution  
{DATABASE=employees.{VAR=user_id}.email}

# Multiple value access
{DATABASE=products.{VAR=sku}.name} - {DATABASE=products.{VAR=sku}.price}
```

**Lookup Process**:
1. Parse database reference into components
2. Load database file (cached in memory)
3. Find row by key column (first column by default)
4. Extract value from specified column
5. Return value or empty string if not found

### Performance Optimizations
- **Memory Mapping**: Large files memory-mapped for efficient access
- **Index Caching**: Primary key index built on first access
- **LRU Cache**: Frequently accessed databases kept in memory
- **Background Loading**: Database loading doesn't block UI

## Translation Services

### DeepL Integration (`translation.rs`)
Poltergeist integrates with DeepL API for inline text translation within snippets.

**Token Syntax**:
```
# Simple translation
{DEEPL=Hello World}=de

# With context
{DEEPL=Please review this document context=document}=fr

# Multiple translations
{DEEPL=Good morning}=es|{DEEPL=Good morning}=fr
```

**Source**: `/crates/poltergeist-io/src/translation.rs` - `DeepLClient` struct

### Translation Process
```rust
// Translation execution
1. Extract text and target language from token
2. Check translation cache (in-memory, disk-persisted)
3. If not cached:
   a. Make async HTTP request to DeepL API
   b. Parse response
   c. Update cache
4. Return translated text
```

**Caching Strategy**:
- **Memory Cache**: Recently used translations
- **Disk Cache**: Persistent cache across sessions  
- **TTL**: Configurable cache expiration
- **Key**: Text + language + context hash

### API Configuration
DeepL API requires authentication via API key:
```json
{
  "services": {
    "deepl": {
      "api_key": "${DEEPL_API_KEY}",
      "endpoint": "https://api.deepl.com/v2/translate",
      "cache_ttl": 86400
    }
  }
}
```

**Environment Variable Support**: `${DEEPL_API_KEY}` resolved from environment

## File I/O Operations

### Path Resolution
Poltergeist uses portable path resolution:
1. **Executable Directory**: Primary location for config and data
2. **User Data Directory**: Fallback location (AppData on Windows)
3. **Team Cache Directory**: Isolated cache for team packs
4. **Database Directory**: Location for registered database files

### File Watching
- **Config File**: Watch for changes and auto-reload
- **Team Packs**: Monitor for external updates
- **Database Files**: Reload on modification
- **Translation Cache**: Periodic cleanup of expired entries

## Async Operations

### Tokio Runtime
The IO crate uses Tokio async runtime for non-blocking operations:

**Async Operations**:
1. HTTP requests (team sync, DeepL translation)
2. File I/O (large database loading)
3. Network operations (UNC share access)
4. Background synchronization

**Source**: `/crates/poltergeist-io/src/lib.rs` - Async function signatures

### Error Handling
```rust
// Comprehensive error handling
pub enum IoError {
    ConfigLoad(ConfigError),
    Network(ReqwestError),
    Database(DatabaseError),
    Translation(TranslationError),
    FileSystem(io::Error),
}
```

**Recovery Strategies**:
- Retry with exponential backoff for network errors
- Fallback to cached data when remote unavailable
- Graceful degradation of features
- User notifications for persistent failures

## Security Considerations

### Sensitive Data Handling
1. **API Keys**: Stored in config with environment variable substitution
2. **Authentication Tokens**: Encrypted storage for team pack auth
3. **Database Contents**: Local files only, no remote database connections
4. **Translation Cache**: Plain text storage (consider encryption for sensitive text)

### Input Validation
1. **Config Files**: Schema validation on load
2. **Team Packs**: Snippet validation before merging
3. **Database Files**: Format validation and sanitization
4. **Translation Input**: Length limits and content filtering

## Performance Optimization

### Caching Layers
1. **Config Cache**: Parsed config in memory
2. **Database Cache**: Indexed database contents
3. **Translation Cache**: Previous translation results
4. **Team Pack Cache**: Remote snippet trees
5. **File System Cache**: Frequently accessed files

### Lazy Loading
- Databases loaded on first access
- Team packs synchronized in background
- Translation cache populated as needed
- UI resources loaded on demand

## Monitoring and Logging

### Operational Metrics
1. **Config Load Times**: Performance tracking
2. **Sync Durations**: Team pack synchronization
3. **Cache Hit Rates**: Effectiveness of caching
4. **Error Rates**: Service reliability
5. **Translation Latency**: DeepL API performance

### Logging Strategy
- Structured logging with tracing crate
- Configurable log levels
- Rotating log files
- Performance tracing for debugging

## Extension Points

### New Configuration Sources
1. **Environment Variables**: Additional override support
2. **Command Line Args**: Runtime configuration
3. **Cloud Config**: Remote configuration services
4. **Registry Integration**: Windows registry (admin edition)

### Additional Service Integrations
1. **Translation Services**: Google Translate, Azure Translator
2. **Database Systems**: SQLite, PostgreSQL connections
3. **Team Collaboration**: Git integration, SharePoint
4. **Cloud Storage**: OneDrive, Google Drive synchronization

### Enhanced Caching
1. **Distributed Cache**: Redis/Memcached for team environments
2. **Predictive Loading**: Anticipate needed databases
3. **Compression**: Compress cached data for disk savings
4. **Encryption**: Secure storage for sensitive snippets