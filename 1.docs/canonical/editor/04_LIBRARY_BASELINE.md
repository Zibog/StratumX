# Editor Library Baseline

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines the allowed editor-side library classes and the canonical host technology decision.
It also freezes which repository-side host experiments are non-authoritative.

## Canonical host decision
The canonical editor host for product implementation is a Rust-native desktop host under `6.apps/editor/`.
That host owns:
- process bootstrap
- native window bootstrap
- device/surface bootstrap
- mounting the editor package
- hosting the primary viewport and product shell

The repository path `8.examples/_obsolete/stratumx_editor_ui_preview_pkg` is archived migration evidence and not canonical product truth.
It may remain as migration evidence, UI experimentation, or comparison material.
It may not define host truth, dto truth, id truth, or viewport truth.

## Scope
The editor package (`L8` through `L11`) may own library classes for:
- UI composition and layout
- render-surface hosting for the honest viewport frame
- editor runtime coordination
- plugin and extension hosting
- serialization of editor-local state (workspace, layout, preferences)
- indexing and search for editor surfaces
- diagnostics presentation and routing
- collaboration session management

The editor package must not own library classes for:
- world truth or entity authority
- asset compilation or transformation
- core simulation or physics
- rendering pipelines or shader compilation
- network transport or protocol
- file system or package management (consumed from tooling)

## Allowed editor library classes
### UI and render-surface classes
- `EditorShell`
- `ViewportSurface`
- `PanelHost`
- `OverlayRenderer`
- `LayoutManager`
- `ThemeProvider`

### Editor runtime classes
- `EditorSession`
- `SelectionManager`
- `FocusRouter`
- `CommandDispatcher`
- `ModeManager`
- `PreviewCoordinator`

### Plugin and extension classes
- `PluginHost`
- `ExtensionRegistry`
- `CapabilityProvider`
- `ScriptBridge`

### Serialization classes
- `WorkspaceSerializer`
- `LayoutSerializer`
- `PreferenceSerializer`

### Indexing and search classes
- `AssetIndexer`
- `EntityIndexer`
- `CommandIndexer`

### Diagnostics classes
- `DiagnosticRouter`
- `DiagnosticPresenter`
- `ValidationCoordinator`

### Collaboration classes
- `CollaborationSession`
- `ReviewCoordinator`
- `PlaytestCoordinator`

## Prohibited library role leakage
The editor package must not:
- own world truth or entity authority;
- own asset compilation or transformation;
- own file system or package management;
- own network transport or protocol;
- own rendering pipelines or shader compilation;
- own simulation or physics;
- maintain shadow truth or parallel state;
- bypass lower-stack authority.

## Concrete external freeze set
Pinned editor-local external baseline:
- `serde`
- `serde_json`
- `thiserror`
- `smallvec`
- `tracing`
- `uuid`
- `parking_lot`
- `winit`
- `egui`
- `egui-winit`
- `egui-wgpu`

Additional third-party libraries are forbidden until they are first declared in this root baseline and then bound to a concrete editor-local role.

## Non-canonical repository stacks
The following libraries may appear in non-authoritative preview code or migration evidence, but they are not part of the canonical product-host baseline:
- `react`
- `react-dom`
- `vite`
- `typescript`
- `tauri`
- `@tauri-apps/api`

Their presence in the repository does not legalize them for product-host ownership.

## Boundary preservation
All editor library classes must:
- consume lower-stack truth via sanctioned surfaces;
- never write directly to lower-stack truth;
- never maintain parallel truth;
- never bypass authority boundaries;
- release resources when surfaces are cold.

## Chrome and windowing extension
The canonical baseline remains Rust-native and egui-hosted.
The shell must still provide:
- saved layouts;
- detachable windows;
- multi-viewport hosts;
- stable panel docking;
- assistant and extension docks.

If the chosen baseline crates do not provide one of those cleanly, the missing behavior must be added through editor-owned shell layers.
It may not be outsourced to a parallel web shell that becomes the real product.

## Embedded-web prohibition
Embedded web surfaces are legal only for:
- docs/help panes;
- isolated extension UIs with explicit mount scope;
- non-authoritative community/help content.

They may not become the primary viewport, shell truth, or canonical stage strip host.
