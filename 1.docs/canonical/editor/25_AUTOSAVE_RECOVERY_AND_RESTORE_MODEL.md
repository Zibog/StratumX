# Editor Autosave, Recovery, and Restore Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose

This document defines the editor-wide autosave, recovery, and restore model: autosave cadence classes, dirty state tracking, crash recovery, restoration boundaries, and temporary artifact legality.

## Scope

The editor package (`L8` through `L11`) provides autosave and crash recovery for all user-visible state.

## Autosave Cadence Classes

### Cadence Structure
```rust
enum AutosaveCadence {
    Disabled,
    OnIdle(Duration),      // e.g., 30 seconds of inactivity
    OnInterval(Duration),  // e.g., every 5 minutes
    OnChange(usize),       // e.g., every 10 changes
}
```

### Cadence Discipline
- Autosave cadence is user-configurable
- Autosave cadence may be different for different surfaces
- Autosave cadence may be disabled for performance-critical workflows

### Cadence Examples
- Workspace layout: OnIdle(30 seconds)
- World state: OnInterval(5 minutes) or OnChange(10 changes)
- Asset state: OnChange(1 change) (immediate)
- Preferences: OnChange(1 change) (immediate)

## Dirty State Tracking

### Dirty State Structure
```rust
struct DirtyState {
    world_dirty: bool,
    assets_dirty: HashSet<AssetId>,
    layout_dirty: bool,
    preferences_dirty: bool,
}
```

### Dirty State Discipline
- Dirty state is tracked per surface
- Dirty state is updated on every mutation
- Dirty state is cleared on save
- Dirty state is used to determine autosave eligibility

### Dirty State UI
- Dirty state is shown in UI (e.g., asterisk in title bar)
- Dirty state is shown in panel tabs (e.g., asterisk in tab label)
- Dirty state is shown in content browser (e.g., asterisk in asset name)

## Crash Recovery

### Recovery Structure
```rust
struct RecoveryState {
    workspace: WorkspaceState,
    layout: LayoutState,
    world: WorldState,
    assets: HashMap<AssetId, AssetState>,
    timestamp: Timestamp,
}
```

### Recovery Discipline
- Recovery state is written to disk on autosave
- Recovery state is written to a temporary location (not the main project)
- Recovery state is detected on editor startup
- Recovery state is presented to the user for restoration

### Recovery UI
- On startup, if recovery state is detected, show recovery dialog
- Recovery dialog shows timestamp and list of recoverable state
- User may choose to restore recovery state or discard it
- User may choose to restore partial state (e.g., layout only)

## Restoration Boundaries

### Restoration Discipline
- Restoration must not corrupt the main project
- Restoration must not overwrite user changes made outside the editor
- Restoration must not restore invalid state
- Restoration must validate state before applying

### Restoration Validation
- Validate workspace schema version
- Validate world state consistency
- Validate asset state consistency
- Validate layout state consistency

### Restoration Conflicts
- If restoration conflicts with current state, present conflict dialog
- User may choose to overwrite current state or cancel restoration
- User may choose to merge restoration state with current state (if possible)

## Temporary Artifact Legality

### Temporary Artifact Structure
```rust
struct TemporaryArtifact {
    path: PathBuf,
    purpose: ArtifactPurpose,
    lifetime: ArtifactLifetime,
}

enum ArtifactPurpose {
    Autosave,
    Recovery,
    Preview,
    Build,
    Export,
}

enum ArtifactLifetime {
    Session,      // Deleted on editor close
    Project,      // Deleted on project close
    Persistent,   // Deleted manually or on cleanup
}
```

### Temporary Artifact Discipline
- Temporary artifacts must be stored in a designated temporary location
- Temporary artifacts must not be stored in the main project
- Temporary artifacts must be cleaned up on editor close (if Session lifetime)
- Temporary artifacts must be cleaned up on project close (if Project lifetime)
- Temporary artifacts must be cleaned up periodically (if Persistent lifetime)

### Temporary Artifact Locations
- Autosave: `<temp>/stratumx/autosave/<project_id>/`
- Recovery: `<temp>/stratumx/recovery/<project_id>/`
- Preview: `<temp>/stratumx/preview/<project_id>/`
- Build: `<temp>/stratumx/build/<project_id>/`
- Export: `<temp>/stratumx/export/<project_id>/`

## Autosave and Collaboration

### Collaboration Discipline
- Autosave is local to each user in a collaboration session
- Autosave does not affect other users' state
- Autosave may conflict with other users' changes (conflict resolution required)

### Conflict Resolution
- Conflicts are detected on autosave
- Conflicts are logged but do not block autosave
- Conflicts are presented to the user on next manual save

## Autosave Performance

### Performance Discipline
- Autosave must not block the UI thread
- Autosave must not cause frame drops in viewport
- Autosave must not cause input lag
- Autosave must be cancellable

### Performance Optimization
- Autosave uses background threads
- Autosave uses incremental serialization (only dirty state)
- Autosave uses compression (if beneficial)
- Autosave uses batching (multiple changes in one autosave)

## Version

This document is part of the `SX-CANON/1.0.24/STACK-v30` editor package.
