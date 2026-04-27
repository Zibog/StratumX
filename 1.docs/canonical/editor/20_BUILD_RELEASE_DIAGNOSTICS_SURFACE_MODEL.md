# Build Release Diagnostics Surface Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Responsibilities
- diagnostics panels and validation dashboards
- console and trace views
- bake queues and build queues
- release target and package views
- artifact status and closure reports
- remediation entry points for validation/build/release failures
- editor performance HUD publication
- world-open, viewport, terrain, sky/environment, host, dto, and runtime-entry posture publication

## Required surfaces
- Validation Dock
- Console / Diagnostics Stream
- Bake / Build Queue View
- Package / Module Dock
- Dependency / Reference Problem View
- Performance / Budget HUD
- Runtime Inspector Dock
- Viewport Failure and Degradation Rail
- World/Open Status Rail

## Mandatory diagnostics classes for first closure
- `HostSelectionDiagnostics`
- `WorldOpenDiagnostics`
- `WorldBindDiagnostics`
- `TerrainBindingDiagnostics`
- `SkyEnvironmentBindingDiagnostics`
- `ViewportFrameDiagnostics`
- `RuntimeEntryDiagnostics`
- `DegradationDiagnostics`
- `BridgeContractDiagnostics`
- `ControlSurfaceDiagnostics`

## Failure-class law
Diagnostics must distinguish at least:
- canonical host mismatch
- startup world missing
- world open rejected
- world bind incomplete
- terrain bind missing
- sky/environment bind missing
- viewport frame unavailable
- runtime entry denied
- unsupported capability tier
- bridge contract mismatch
- mixed identity family
- dead or disabled control path

## Laws
- diagnostics surfaces display lower-stack projections only;
- build/release surfaces host requests, presets, and queue views; they do not own packaging logic;
- bake/build/release failures must remain visible until acknowledged or superseded;
- diagnostics must explain why the first product-result loop is blocked whenever it is blocked;
- diagnostics are part of product truth; they are not optional chrome.
