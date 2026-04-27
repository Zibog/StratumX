# Editor Communication Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose

This document defines the editor-wide communication model: shell commands, panel events, selection/focus changes, requests/results, and message classes.

## Scope

The editor package (`L8` through `L11`) uses structured communication for:
- Shell commands (command palette, shortcuts, menu)
- Panel and view events (open, close, focus, blur, dock, undock)
- Selection and focus changes (cross-surface selection coordination)
- Request/result flows (preview, diagnostics, build, assistant, collaboration)
- Invalidation and refresh signals (viewport, panel, overlay)

## Shell Commands

### Command Structure
```rust
struct EditorCommand {
    id: CommandId,
    category: CommandCategory,
    label: String,
    shortcut: Option<Shortcut>,
    enabled: bool,
    handler: CommandHandler,
}
```

### Command Categories
- `File`: New, open, save, close, import, export
- `Edit`: Undo, redo, cut, copy, paste, delete
- `View`: Show/hide panels, toggle overlays, change viewport mode
- `Select`: Select all, deselect, invert selection, select by type
- `Tool`: Activate tool, change mode, configure tool
- `Build`: Build, rebuild, clean, release
- `Debug`: Play, simulate, pause, stop, step
- `Collaborate`: Start session, join session, review, annotate
- `Assistant`: Ask, suggest, automate
- `Help`: Documentation, tutorials, about

### Command Routing
- Commands are dispatched via `CommandDispatcher`
- Commands may be enabled/disabled based on editor state
- Commands may be context-sensitive (e.g., "Delete" requires selection)
- Commands may be async (e.g., "Build" returns a future)

## Panel and View Events

### Event Structure
```rust
enum PanelEvent {
    Opened(PanelId),
    Closed(PanelId),
    Focused(PanelId),
    Blurred(PanelId),
    Docked(PanelId, DockPosition),
    Undocked(PanelId),
    Resized(PanelId, Size),
}
```

### Event Routing
- Panel events are routed via `PanelHost`
- Panel events may trigger layout persistence
- Panel events may trigger resource activation/deactivation
- Panel events may trigger invalidation/refresh

## Selection and Focus Changes

### Selection Structure
```rust
struct Selection {
    entities: Vec<EntityId>,
    assets: Vec<AssetId>,
    components: Vec<ComponentId>,
    source: SelectionSource,
}
```

### Selection Routing
- Selection changes are routed via `SelectionManager`
- Selection changes may trigger inspector refresh
- Selection changes may trigger viewport highlight
- Selection changes may trigger outliner scroll
- Selection is cross-surface: viewport, outliner, inspector, content browser

### Focus Structure
```rust
struct Focus {
    panel: Option<PanelId>,
    viewport: Option<ViewportId>,
    input_target: Option<InputTargetId>,
}
```

### Focus Routing
- Focus changes are routed via `FocusRouter`
- Focus changes may trigger input routing
- Focus changes may trigger shortcut context
- Focus changes may trigger tool mode

## Request/Result Flows

### Preview Request/Result
```rust
struct PreviewRequest {
    asset: AssetId,
    viewport: ViewportId,
    mode: PreviewMode,
}

struct PreviewResult {
    asset: AssetId,
    viewport: ViewportId,
    status: PreviewStatus,
    error: Option<String>,
}
```

### Diagnostics Request/Result
```rust
struct DiagnosticsRequest {
    scope: DiagnosticsScope,
    severity: DiagnosticsSeverity,
}

struct DiagnosticsResult {
    messages: Vec<DiagnosticMessage>,
    status: DiagnosticsStatus,
}
```

### Build Request/Result
```rust
struct BuildRequest {
    target: BuildTarget,
    configuration: BuildConfiguration,
}

struct BuildResult {
    target: BuildTarget,
    status: BuildStatus,
    artifacts: Vec<ArtifactPath>,
    errors: Vec<BuildError>,
}
```

### Assistant Request/Result
```rust
struct AssistantRequest {
    prompt: String,
    context: AssistantContext,
}

struct AssistantResult {
    response: String,
    actions: Vec<AssistantAction>,
    status: AssistantStatus,
}
```

### Collaboration Request/Result
```rust
struct CollaborationRequest {
    session: SessionId,
    operation: CollaborationOperation,
}

struct CollaborationResult {
    session: SessionId,
    status: CollaborationStatus,
    error: Option<String>,
}
```

## Invalidation and Refresh Signals

### Invalidation Structure
```rust
enum InvalidationSignal {
    WorldChanged(Vec<EntityId>),
    AssetChanged(Vec<AssetId>),
    SelectionChanged(Selection),
    LayoutChanged(LayoutId),
    ModeChanged(ModeId),
}
```

### Refresh Routing
- Invalidation signals are routed to affected surfaces
- Surfaces may batch invalidation signals
- Surfaces may defer refresh until visible
- Surfaces may release resources when cold

## Message Classes

All editor communication uses typed message classes:
- No stringly-typed messages
- No untyped payloads
- No hidden side effects
- No bypassing authority boundaries

## Version

This document is part of the `SX-CANON/1.0.24/STACK-v30` editor package.
