# Editor Workspace Schema and Migration Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose

This document defines the editor-wide workspace schema and migration model: schema versioning, migration policy, compatibility posture, and migration staging and validation.

## Scope

The editor package (`L8` through `L11`) owns workspace schema and migration for editor-local state (layout, preferences, recent projects).

## Workspace Schema Versioning

### Schema Structure
```rust
struct WorkspaceSchema {
    version: SchemaVersion,
    layout: LayoutSchema,
    preferences: PreferencesSchema,
    recent_projects: RecentProjectsSchema,
}

struct SchemaVersion {
    major: u32,
    minor: u32,
    patch: u32,
}
```

### Versioning Discipline
- Workspace schema has a semantic version (major.minor.patch)
- Major version changes indicate breaking changes (migration required)
- Minor version changes indicate backward-compatible additions (no migration required)
- Patch version changes indicate bug fixes (no migration required)

### Versioning Examples
- `1.0.0` → `1.1.0`: Added new preference (backward-compatible, no migration)
- `1.1.0` → `2.0.0`: Changed layout schema (breaking change, migration required)
- `2.0.0` → `2.0.1`: Fixed preference serialization bug (no migration)

## Migration Policy

### Migration Discipline
- Migration is automatic on workspace load
- Migration is one-way (no downgrade)
- Migration is validated before applying
- Migration is logged for debugging

### Migration Triggers
- Workspace schema version < editor schema version: Migration required
- Workspace schema version = editor schema version: No migration required
- Workspace schema version > editor schema version: Error (workspace created by newer editor)

### Migration Errors
- If migration fails, show error dialog
- If migration fails, offer to reset workspace to default
- If migration fails, offer to backup workspace before reset

## Compatibility Posture

### Forward Compatibility
- The editor does NOT support forward compatibility
- Workspaces created by newer editors cannot be opened by older editors
- Show error dialog if workspace schema version > editor schema version

### Backward Compatibility
- The editor supports backward compatibility for N-1 major versions
- Workspaces created by older editors are migrated automatically
- Show warning dialog if workspace schema version is very old (> N-1 major versions)

### Compatibility Examples
- Editor 2.0.0 can open workspaces from 1.x.x (backward-compatible)
- Editor 1.x.x cannot open workspaces from 2.0.0 (not forward-compatible)
- Editor 3.0.0 can open workspaces from 2.x.x but not 1.x.x (N-1 major versions)

## Migration Staging

### Staging Discipline
- Migration is staged in multiple steps
- Each step is validated before proceeding
- Each step is logged for debugging
- Each step is reversible (if possible)

### Staging Steps
1. Backup workspace (to temporary location)
2. Validate workspace schema version
3. Load workspace with old schema
4. Transform workspace to new schema
5. Validate workspace with new schema
6. Save workspace with new schema
7. Delete backup (if migration succeeded)

### Staging Errors
- If any step fails, revert to backup
- If any step fails, show error dialog
- If any step fails, log error for debugging

## Migration Validation

### Validation Discipline
- Validation is performed before and after migration
- Validation checks schema version, structure, and constraints
- Validation checks for missing or invalid fields
- Validation checks for data corruption

### Validation Examples
- Validate layout schema: All panels have valid IDs, all viewports have valid modes
- Validate preferences schema: All preferences have valid types and values
- Validate recent projects schema: All projects have valid paths

### Validation Errors
- If validation fails before migration, show error dialog (workspace is corrupted)
- If validation fails after migration, revert to backup (migration failed)

## Workspace Schema Examples

### Layout Schema
```rust
struct LayoutSchema {
    version: SchemaVersion,
    windows: Vec<WindowLayout>,
    active_window: WindowId,
}

struct WindowLayout {
    id: WindowId,
    panels: Vec<PanelLayout>,
    viewports: Vec<ViewportLayout>,
}
```

### Preferences Schema
```rust
struct PreferencesSchema {
    version: SchemaVersion,
    theme: ThemePreference,
    shortcuts: ShortcutPreferences,
    autosave: AutosavePreferences,
    collaboration: CollaborationPreferences,
}
```

### Recent Projects Schema
```rust
struct RecentProjectsSchema {
    version: SchemaVersion,
    projects: Vec<RecentProject>,
}

struct RecentProject {
    path: PathBuf,
    name: String,
    last_opened: Timestamp,
}
```

## Migration and Collaboration

### Collaboration Discipline
- Migration is local to each user
- Migration does not affect other users' workspaces
- Migration does not affect project state (only workspace state)

## Version

This document is part of the `SX-CANON/1.0.24/STACK-v30` editor package.
