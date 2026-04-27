# Code Cleanup Guide

This guide documents the process for identifying and remediating architectural debt in the StratumX game engine repository as part of Phase 1 repository sanitization.

## Overview

The code cleanup system identifies three categories of architectural debt:

1. **Host Bypasses**: Direct host service access from UI code without proper routing
2. **Registration Blobs**: Large monolithic registration modules mixing multiple concerns
3. **Domain Logic in UI**: Business logic, parsers, or validators in application layer

These issues require manual refactoring to maintain architectural boundaries and ensure long-term maintainability.

## Cleanup Process

### Phase 1: Identification

Run the cleanup detection system to generate a report:

```bash
cargo run -p stratumx_quality_tasks -- clean --report-only
```

This scans the repository and generates a comprehensive report of all issues requiring manual refactoring.

### Phase 2: Prioritization

Review the cleanup report and prioritize issues based on:

1. **Severity**: How much the issue violates architectural boundaries
2. **Impact**: How many other components depend on the problematic code
3. **Effort**: How much work is required to remediate
4. **Risk**: How likely the refactoring is to introduce bugs

### Phase 3: Remediation

For each identified issue, follow the appropriate remediation strategy (detailed below).

### Phase 4: Verification

After remediation:

1. Run hygiene checks to verify the issue is resolved
2. Run tests to ensure functionality is preserved
3. Run cleanup detection again to confirm the issue is gone

## Running Cleanup Detection

### Generate Cleanup Report

```bash
cargo run -p stratumx_quality_tasks -- clean --report-only
```

**Output**:
```
Generating cleanup report...
REPORT ONLY MODE - No changes will be made

=== Cleanup Report ===

Host Bypasses Found: 2
  File: 6.apps/editor/stratumx_editor_app/src/desktop_app/project_panel.rs
  Line: 145
  Pattern: self.host.save_file(path, content)
  Action: Route through Command_Spine SaveFile action

  File: 6.apps/editor/stratumx_editor_app/src/desktop_app/asset_panel.rs
  Line: 203
  Pattern: self.host.open_file_dialog()
  Action: Route through Command_Spine OpenFileDialog action

Registration Blobs Found: 1
  File: 5.editor/l7.0-editor-command-spine/src/registration.rs
  Lines: 347
  Mixed Concerns: command_registration, menu_registration, keybinding_registration
  Suggested Decomposition:
    - Extract command_registration to separate module
    - Extract menu_registration to separate module
    - Extract keybinding_registration to separate module

Domain Logic Violations Found: 1
  File: 6.apps/editor/stratumx_editor_app/src/desktop_app/asset_panel.rs
  Type: Parser
  Lines: 89-145
  Target Layer: 4.tooling/asset-parser or appropriate service layer

Total issues found: 4

These issues require manual refactoring.
```

### Understanding the Report

The cleanup report provides:

- **File location**: Exact path to the file containing the issue
- **Line number(s)**: Where the issue occurs
- **Pattern/Type**: What kind of issue was detected
- **Suggested action**: Recommended remediation approach
- **Target layer**: Where the code should be moved (for domain logic violations)

## Issue Types and Remediation

### 1. Host Bypasses

#### What Are Host Bypasses?

Host bypasses occur when UI code directly accesses host services (file system, dialogs, etc.) without routing through the canonical action path (Command_Spine).

#### Why Are They Problematic?

- Violates architectural boundaries
- Bypasses command history and undo/redo
- Makes testing difficult
- Prevents proper error handling and validation
- Breaks the separation between UI and business logic

#### Example Violation

```rust
// In 6.apps/editor/stratumx_editor_app/src/desktop_app/project_panel.rs
impl ProjectPanel {
    fn save_project(&mut self) {
        // VIOLATION: Direct host access
        self.host.save_file(&self.project_path, &self.project_data);
    }
}
```

#### Remediation Strategy

**Step 1: Define a Command**

Create a command in Command_Spine:

```rust
// In 5.editor/l7.0-editor-command-spine/src/commands/save_project.rs
pub struct SaveProjectCommand {
    pub path: PathBuf,
    pub data: ProjectData,
}

impl Command for SaveProjectCommand {
    fn execute(&self, context: &mut CommandContext) -> Result<(), CommandError> {
        // Proper validation, error handling, and host access
        context.host.save_file(&self.path, &self.data)?;
        context.history.record(self);
        Ok(())
    }
}
```

**Step 2: Route Through Command_Spine**

Update the UI code to dispatch the command:

```rust
// In 6.apps/editor/stratumx_editor_app/src/desktop_app/project_panel.rs
impl ProjectPanel {
    fn save_project(&mut self) {
        // CORRECT: Route through Command_Spine
        let command = SaveProjectCommand {
            path: self.project_path.clone(),
            data: self.project_data.clone(),
        };
        self.command_spine.dispatch(command);
    }
}
```

**Step 3: Verify**

1. Run tests to ensure functionality is preserved
2. Run cleanup detection to verify the bypass is gone
3. Test undo/redo functionality

#### Common Host Bypass Patterns

| Pattern | Remediation |
|---------|-------------|
| `self.host.save_file(...)` | Route through `SaveFileCommand` |
| `self.host.open_file_dialog()` | Route through `OpenFileDialogCommand` |
| `self.host.load_file(...)` | Route through `LoadFileCommand` |
| `self.host.create_directory(...)` | Route through `CreateDirectoryCommand` |
| `self.host.delete_file(...)` | Route through `DeleteFileCommand` |

### 2. Registration Blobs

#### What Are Registration Blobs?

Registration blobs are large monolithic modules (>200 lines) that mix multiple concerns in a single file, typically found in Command_Spine registration code.

#### Why Are They Problematic?

- Difficult to understand and maintain
- Mix multiple concerns (commands, menus, keybindings, etc.)
- Hard to test individual concerns
- Violate single responsibility principle
- Make code review difficult

#### Example Violation

```rust
// In 5.editor/l7.0-editor-command-spine/src/registration.rs (347 lines)
pub fn register_all(spine: &mut CommandSpine) {
    // Command registration (lines 1-120)
    spine.register_command("save", SaveCommand::new());
    spine.register_command("open", OpenCommand::new());
    // ... 100+ more commands
    
    // Menu registration (lines 121-240)
    spine.register_menu("File", FileMenu::new());
    spine.register_menu("Edit", EditMenu::new());
    // ... 100+ more menus
    
    // Keybinding registration (lines 241-347)
    spine.register_keybinding("Ctrl+S", "save");
    spine.register_keybinding("Ctrl+O", "open");
    // ... 100+ more keybindings
}
```

#### Remediation Strategy

**Step 1: Identify Concerns**

Analyze the blob to identify distinct concerns:
- Command registration
- Menu registration
- Keybinding registration
- Toolbar registration
- etc.

**Step 2: Extract Each Concern**

Create separate modules for each concern:

```rust
// In 5.editor/l7.0-editor-command-spine/src/registration/commands.rs
pub fn register_commands(spine: &mut CommandSpine) {
    spine.register_command("save", SaveCommand::new());
    spine.register_command("open", OpenCommand::new());
    // ... all command registrations
}

// In 5.editor/l7.0-editor-command-spine/src/registration/menus.rs
pub fn register_menus(spine: &mut CommandSpine) {
    spine.register_menu("File", FileMenu::new());
    spine.register_menu("Edit", EditMenu::new());
    // ... all menu registrations
}

// In 5.editor/l7.0-editor-command-spine/src/registration/keybindings.rs
pub fn register_keybindings(spine: &mut CommandSpine) {
    spine.register_keybinding("Ctrl+S", "save");
    spine.register_keybinding("Ctrl+O", "open");
    // ... all keybinding registrations
}
```

**Step 3: Create Coordinator Module**

Create a thin coordinator that calls each registration module:

```rust
// In 5.editor/l7.0-editor-command-spine/src/registration/mod.rs
mod commands;
mod menus;
mod keybindings;

pub fn register_all(spine: &mut CommandSpine) {
    commands::register_commands(spine);
    menus::register_menus(spine);
    keybindings::register_keybindings(spine);
}
```

**Step 4: Verify**

1. Ensure each module is under 200 lines
2. Run tests to ensure all registrations still work
3. Run cleanup detection to verify the blob is gone

#### Decomposition Guidelines

- **Each module should handle a single concern**
- **Keep modules under 200 lines**
- **Use descriptive module names** (e.g., `command_registration`, not `reg1`)
- **Group related registrations** (e.g., all file commands together)
- **Document the purpose** of each module

### 3. Domain Logic in UI

#### What Is Domain Logic in UI?

Domain logic in UI occurs when business logic, parsers, validators, or other domain-specific code is implemented directly in application layer panels or UI components.

#### Why Is It Problematic?

- Violates separation of concerns
- Makes UI code complex and hard to maintain
- Prevents reuse of domain logic
- Makes testing difficult
- Couples UI to business rules

#### Example Violations

**Parser in UI**:
```rust
// In 6.apps/editor/stratumx_editor_app/src/desktop_app/asset_panel.rs
impl AssetPanel {
    fn parse_asset_file(&self, content: &str) -> Result<Asset, ParseError> {
        // VIOLATION: Parser implementation in UI
        let lines = content.lines();
        let mut asset = Asset::default();
        for line in lines {
            if line.starts_with("name:") {
                asset.name = line[5..].trim().to_string();
            }
            // ... 50+ more lines of parsing logic
        }
        Ok(asset)
    }
}
```

**Validator in UI**:
```rust
// In 6.apps/editor/stratumx_editor_app/src/desktop_app/project_panel.rs
impl ProjectPanel {
    fn validate_project_name(&self, name: &str) -> bool {
        // VIOLATION: Validation logic in UI
        !name.is_empty() 
            && name.len() <= 64 
            && name.chars().all(|c| c.is_alphanumeric() || c == '_')
            && !name.starts_with(char::is_numeric)
    }
}
```

**Business Rule in UI**:
```rust
// In 6.apps/editor/stratumx_editor_app/src/desktop_app/scene_panel.rs
impl ScenePanel {
    fn can_add_entity(&self) -> bool {
        // VIOLATION: Business rule in UI
        self.entity_count < self.max_entities 
            && self.memory_usage < self.memory_limit
            && self.has_valid_scene()
    }
}
```

#### Remediation Strategy

**Step 1: Identify the Domain Logic**

Determine what kind of domain logic is in the UI:
- Parser
- Validator
- Business rule
- Data transformation
- Algorithm implementation

**Step 2: Determine the Target Layer**

Based on the type of logic, determine where it should live:

| Logic Type | Target Layer | Example Location |
|------------|--------------|------------------|
| Parser | Tooling | `4.tooling/asset-parser/` |
| Validator | SDK | `3.sdk/validation/` |
| Business Rule | Editor Services | `5.editor/services/` |
| Data Transformation | SDK | `3.sdk/data-transform/` |
| Algorithm | Engine | `2.engine/algorithms/` |

**Step 3: Extract to Service Layer**

Create a service in the appropriate layer:

```rust
// In 4.tooling/asset-parser/src/asset_parser.rs
pub struct AssetParser;

impl AssetParser {
    pub fn parse(&self, content: &str) -> Result<Asset, ParseError> {
        let lines = content.lines();
        let mut asset = Asset::default();
        for line in lines {
            if line.starts_with("name:") {
                asset.name = line[5..].trim().to_string();
            }
            // ... parsing logic
        }
        Ok(asset)
    }
}
```

**Step 4: Update UI to Use Service**

```rust
// In 6.apps/editor/stratumx_editor_app/src/desktop_app/asset_panel.rs
use asset_parser::AssetParser;

impl AssetPanel {
    fn load_asset(&mut self, content: &str) {
        // CORRECT: Use service layer
        let parser = AssetParser;
        match parser.parse(content) {
            Ok(asset) => self.display_asset(asset),
            Err(e) => self.show_error(e),
        }
    }
}
```

**Step 5: Verify**

1. Run tests to ensure functionality is preserved
2. Run cleanup detection to verify the violation is gone
3. Verify the service can be reused in other contexts

#### UI Responsibilities

The UI layer should only:
- **Display data** provided by services
- **Capture user input** and pass it to services
- **Coordinate UI state** (e.g., which panel is visible)
- **Handle UI-level validation** (e.g., "field is required")
- **Route actions** through Command_Spine

The UI layer should NOT:
- Parse file formats
- Validate business rules
- Implement algorithms
- Transform data structures
- Access host services directly

## Interpreting Cleanup Reports

### Host Bypass Report Entry

```
File: 6.apps/editor/stratumx_editor_app/src/desktop_app/project_panel.rs
Line: 145
Pattern: self.host.save_file(path, content)
Action: Route through Command_Spine SaveFile action
```

**Interpretation**:
- **File**: The file containing the violation
- **Line**: The exact line number
- **Pattern**: The specific code pattern detected
- **Action**: The recommended remediation approach

### Registration Blob Report Entry

```
File: 5.editor/l7.0-editor-command-spine/src/registration.rs
Lines: 347
Mixed Concerns: command_registration, menu_registration, keybinding_registration
Suggested Decomposition:
  - Extract command_registration to separate module
  - Extract menu_registration to separate module
  - Extract keybinding_registration to separate module
```

**Interpretation**:
- **File**: The file containing the blob
- **Lines**: Total line count (exceeds 200 line limit)
- **Mixed Concerns**: The distinct concerns identified in the blob
- **Suggested Decomposition**: Specific modules to extract

### Domain Logic Violation Report Entry

```
File: 6.apps/editor/stratumx_editor_app/src/desktop_app/asset_panel.rs
Type: Parser
Lines: 89-145
Target Layer: 4.tooling/asset-parser or appropriate service layer
```

**Interpretation**:
- **File**: The file containing the violation
- **Type**: The kind of domain logic detected (Parser, Validator, BusinessRule)
- **Lines**: The line range where the violation occurs
- **Target Layer**: Where the logic should be moved

## Remediation Workflow

### 1. Review the Report

```bash
cargo run -p stratumx_quality_tasks -- clean --report-only > cleanup_report.txt
```

Review the report and create a prioritized list of issues to address.

### 2. Create Tracking Issues

For each issue, create a tracking issue in your issue tracker with:
- Description of the violation
- Proposed remediation approach
- Estimated effort
- Priority

### 3. Implement Remediation

For each issue:

1. **Create a branch**: `git checkout -b fix/cleanup-<issue-number>`
2. **Implement the fix**: Follow the appropriate remediation strategy
3. **Write tests**: Ensure the refactored code is tested
4. **Verify**: Run hygiene checks and cleanup detection
5. **Commit**: `git commit -m "Fix cleanup issue #<number>: <description>"`
6. **Submit for review**: Create a pull request

### 4. Verify Resolution

After merging the fix:

```bash
# Run cleanup detection
cargo run -p stratumx_quality_tasks -- clean --report-only

# Run hygiene checks
cargo run -p stratumx_quality_tasks -- verify

# Run tests
cargo test --workspace
```

Verify that:
- The issue no longer appears in the cleanup report
- All hygiene checks pass
- All tests pass

## Best Practices

### Before Remediation

1. **Understand the code**: Read and understand the code before refactoring
2. **Identify dependencies**: Determine what other code depends on the problematic code
3. **Write tests**: If tests don't exist, write them before refactoring
4. **Create a branch**: Always work in a feature branch

### During Remediation

1. **Make small changes**: Refactor incrementally, not all at once
2. **Run tests frequently**: Verify functionality after each change
3. **Preserve behavior**: Ensure the refactored code behaves identically
4. **Document decisions**: Add comments explaining non-obvious design choices

### After Remediation

1. **Run full test suite**: Ensure nothing broke
2. **Run hygiene checks**: Verify the issue is resolved
3. **Update documentation**: Update any affected documentation
4. **Code review**: Have the changes reviewed by another developer

## Common Pitfalls

### Over-Refactoring

**Problem**: Refactoring too much at once, introducing bugs or breaking changes

**Solution**: Make small, incremental changes and verify after each step

### Under-Refactoring

**Problem**: Partially fixing the issue, leaving some violations in place

**Solution**: Use cleanup detection to verify the issue is completely resolved

### Breaking Dependencies

**Problem**: Refactoring breaks other code that depends on the problematic code

**Solution**: Identify all dependencies before refactoring and update them together

### Changing Behavior

**Problem**: Refactored code behaves differently than original code

**Solution**: Write comprehensive tests before refactoring and ensure they all pass

## Continuous Improvement

### Regular Cleanup Detection

Run cleanup detection regularly (e.g., weekly) to catch new issues early:

```bash
cargo run -p stratumx_quality_tasks -- clean --report-only
```

### CI Integration

Integrate cleanup detection into CI to prevent new violations:

```yaml
# Example CI configuration
- name: Run Cleanup Detection
  run: |
    cargo run -p stratumx_quality_tasks -- clean --report-only > cleanup_report.txt
    # Fail if new violations are introduced (optional)
```

### Tracking Progress

Track cleanup progress over time:

1. Run cleanup detection and save the report
2. Count total issues
3. Track reduction in issues over time
4. Celebrate progress!

## References

- **Hygiene Suite README**: `7.quality/suites/repo_hygiene/README.md`
- **Migration Guide**: `7.quality/docs/MIGRATION_GUIDE.md`
- **Requirements**: `.kiro/specs/repo-sanitization-phase-1/requirements.md`
- **Design**: `.kiro/specs/repo-sanitization-phase-1/design.md`
