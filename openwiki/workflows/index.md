# User Workflows

This section describes how users interact with Poltergeist, covering setup, daily usage, team collaboration, and troubleshooting workflows.

## Initial Setup Workflow

### First-Time Installation
1. **Download & Extract**: Get Poltergeist portable zip and extract to desired location
2. **Run Executable**: Launch `poltergeist.exe` (may require admin privileges for certain features)
3. **System Tray**: App runs in system tray with icon indicating status
4. **Initial Configuration**: First run creates default `poltergeist.json` configuration file

### Configuration Workflow
1. **Access Settings**: Right-click tray icon → "Options"
2. **Configure Hotkey**: Set global activation hotkey (default: Ctrl+Alt+Space)
3. **Set Context Extraction**: Define regex patterns for extracting variables from selected text
4. **Configure Team Packs**: Add shared snippet repositories (local/UNC or HTTP)
5. **Register Databases**: Add CSV/XLSX files for `{DATABASE=...}` lookups
6. **Set Preferences**: Language, theme, injection defaults

## Daily Usage Workflow

### Basic Snippet Activation
1. **Select Context**: Highlight text in any application (ticket number, hostname, etc.)
2. **Press Hotkey**: Ctrl+Alt+Space (or configured hotkey)
3. **Choose Snippet**: Nested popup appears at cursor with filtered snippets
4. **Inject Content**: Selected snippet expands tokens and injects into focused field

### Snippet Creation Workflow
1. **Open Editor**: Tray icon → "Edit Snippets"
2. **Create New**:
   - **Snippet**: Define name, template body, injection mode, filters
   - **Folder**: Create organizational containers with nesting support
3. **Configure Filters**: Set "Show when..." rules based on window context
4. **Assign Hotkey**: Optional direct activation hotkey for folder/snippet
5. **Test**: Use hotkey to verify snippet appears and injects correctly

### Template Development Workflow
1. **Identify Pattern**: Determine repeating text patterns in daily work
2. **Add Variables**: Use `{VAR=name}` for dynamic parts extracted from context
3. **Add Logic**: Use `{IF}/{ELSIF}/{ELSE}/{END}` for conditional text
4. **Add External Data**: Use `{DATABASE=...}` for lookups from CSV/XLSX
5. **Add Translation**: Use `{DEEPL=...}` for automatic translation
6. **Test Iteratively**: Refine template with real-world usage

## Context-Aware Workflow

### Context Extraction Setup
1. **Analyze Work Patterns**: Identify common text patterns in daily work
2. **Create Regex Patterns**: 
   ```regex
   # Example: Extract ticket and component
   (?P<ticket>[A-Z]+-\d+)-(?P<component>[A-Za-z]+)
   ```
3. **Test Extraction**: Select matching text, check if variables populate
4. **Create Context-Dependent Snippets**: Use `{VAR=...}` tokens and filter rules

### Smart Filtering Workflow
1. **Window-Specific Snippets**: Create snippets that only show in specific applications
2. **Process-Based Filtering**: Show snippets only for certain executables
3. **Variable-Based Filtering**: Show snippets based on extracted context values
4. **Combination Rules**: Multiple filter conditions using AND/OR logic

## Team Collaboration Workflow

### Team Pack Setup
1. **Choose Sharing Method**:
   - **Local/UNC**: Network share accessible to team
   - **HTTP(S)**: Web endpoint for remote teams
2. **Configure Pack**:
   ```json
   {
     "name": "Engineering",
     "type": "http",
     "url": "https://team.example.com/snippets.json",
     "cache_ttl": 300
   }
   ```
3. **Sync Settings**: Set cache TTL and update frequency
4. **Test Access**: Verify snippets load from team pack

### Team Snippet Development
1. **Shared Structure**: Organize team snippets in logical folders
2. **Documentation**: Include comments in snippet bodies for team members
3. **Version Control**: Team packs should be version-controlled (git, etc.)
4. **Update Process**: Team lead updates pack, members auto-sync on next use

### Conflict Resolution
1. **Personal Overrides**: Personal snippets with same name override team snippets
2. **Cache Fallback**: Use cached version if team pack unavailable
3. **Manual Refresh**: Force sync from tray menu if needed

## Database Integration Workflow

### Database Setup
1. **Prepare Data**: CSV or Excel files with lookup data
2. **Register Database**: Add to configuration with name and path
3. **Verify Structure**: Ensure proper headers/key columns
4. **Test Lookups**: Use `{DATABASE=name.key}` in snippets

### Dynamic Lookup Workflow
1. **Extract Key**: Use context extraction to get lookup key
2. **Reference in Snippet**: `{DATABASE=employees.{VAR=user_id}.email}`
3. **Fallback Handling**: Consider what happens if lookup fails
4. **Multiple Values**: Combine multiple database fields in one snippet

## Translation Workflow

### Translation Setup
1. **Get DeepL API Key**: Register at DeepL.com
2. **Configure Service**: Add API key to configuration (environment variable recommended)
3. **Test Translation**: Create snippet with `{DEEPL=test}=de`

### Multilingual Snippet Workflow
1. **Identify Translatable Content**: Text that needs localization
2. **Add Translation Tokens**: `{DEEPL=Original text}=target_lang`
3. **Context-Aware Translation**: Include context hints for better accuracy
4. **Cache Management**: Translations cached locally for performance

## Advanced Workflows

### Nested Folder Navigation
1. **Deep Organization**: Unlimited folder nesting for complex snippet libraries
2. **Folder Hotkeys**: Assign direct hotkeys to frequently used folders
3. **Quick Access**: Bypass nested menus with folder hotkeys
4. **Visual Organization**: Use icons and naming conventions

### Injection Mode Selection
1. **Per-Snippet Configuration**: Choose optimal injection mode for each snippet
2. **Application Testing**: Test different modes with target applications
3. **Fallback Strategy**: Configure alternative modes for problematic apps
4. **Mode Documentation**: Note which mode works best for each application

### Macro Snippet Workflow
1. **Complex Sequences**: Combine multiple actions in one snippet
2. **Wait Tokens**: Use `{WAIT=ms}` for timing between actions
3. **Key Combinations**: Simulate complex key sequences with `{KEY=...}` tokens
4. **Conditional Execution**: Vary actions based on context

## Troubleshooting Workflow

### Common Issues & Solutions

#### Hotkey Not Working
1. **Check Conflicts**: Other applications may be using same hotkey
2. **Admin Privileges**: Admin edition may be needed for system-wide hotkeys
3. **Focus Issues**: Ensure target application window is active
4. **Restart App**: Sometimes hotkey registration needs refresh

#### Snippet Not Appearing
1. **Filter Check**: Verify "Show when..." rules match current context
2. **Context Extraction**: Ensure selected text matches regex patterns
3. **Folder Visibility**: Parent folder filters may be hiding snippet
4. **Team Pack Sync**: Check if team pack is loaded and accessible

#### Injection Not Working
1. **Mode Selection**: Try different injection modes
2. **Focus Verification**: Ensure target window retains focus
3. **Clipboard Issues**: Check clipboard access permissions
4. **Application Compatibility**: Some apps block certain injection methods

#### Team Pack Sync Issues
1. **Network Access**: Verify connectivity to pack source
2. **Cache Check**: Check local cache directory for errors
3. **Authentication**: Ensure API keys/tokens are valid
4. **Format Validation**: Verify pack JSON format is correct

### Debug Mode
1. **Enable Logging**: Set `RUST_LOG=debug` environment variable
2. **Check Logs**: Review `%APPDATA%\Poltergeist\logs\` directory
3. **Window Spy**: Use included tools to inspect window context
4. **Step-by-Step Testing**: Isolate which part of workflow is failing

## Maintenance Workflow

### Regular Maintenance
1. **Snippet Cleanup**: Review and remove unused snippets
2. **Folder Reorganization**: Restructure for better navigation
3. **Template Updates**: Update templates for changing workflows
4. **Database Updates**: Refresh lookup data from source systems

### Backup & Recovery
1. **Config Backup**: Regularly backup `poltergeist.json`
2. **Snippet Export**: Export snippets for version control
3. **Team Pack Backup**: Ensure team packs are version-controlled
4. **Recovery Test**: Periodically test restoration from backups

### Performance Optimization
1. **Large Library Management**: Organize snippets for quick filtering
2. **Database Indexing**: Ensure frequently used databases are indexed
3. **Cache Management**: Monitor and clear translation/team caches if needed
4. **Startup Optimization**: Review startup time and dependencies

## Training & Onboarding

### New User Onboarding
1. **Basic Setup**: Install, configure hotkey, create first snippet
2. **Context Awareness**: Teach context extraction and filtering
3. **Template Language**: Introduce basic tokens and conditionals
4. **Advanced Features**: Gradual introduction of databases, translation, team packs

### Team Training
1. **Shared Standards**: Establish naming conventions and organization
2. **Best Practices**: Document team-specific workflow patterns
3. **Quality Control**: Review and refine team snippets
4. **Knowledge Sharing**: Regular team snippet reviews and showcases

### Documentation
1. **Internal Wiki**: Team-specific snippet documentation
2. **Snippet Comments**: Document complex snippets within template bodies
3. **Troubleshooting Guide**: Team-specific issue resolution
4. **Update Log**: Track changes to team packs and configurations

## Migration Workflows

### From Other Tools
1. **GhostWriter Migration**: Import snippets from GhostWriter format
2. **PhraseExpress Migration**: Convert PhraseExpress snippets
3. **Text Expander Migration**: Adapt Text Expander snippets
4. **Custom Script Migration**: Convert automation scripts to snippets

### Version Upgrades
1. **Backup Current**: Export all snippets and configuration
2. **Test New Version**: Verify compatibility with existing workflows
3. **Gradual Migration**: Phase in new features while maintaining old workflows
4. **Team Coordination**: Coordinate upgrades across team members