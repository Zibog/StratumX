# Shared Type Registry

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

This document defines the editor-wide shared type registry and the frozen semantic DTO families for the first honest product spine.
Field casing may vary by transport adapter.
Field meaning and field set may not.

## Canonical identity family
```rust
struct StableEntityId(Uuid);
struct StableComponentId(Uuid);
struct StableWorldId(Uuid);
struct StartupWorldRef(Uuid);
struct ValidationWorldRef(Uuid);
struct DemoWorldRef(Uuid);
struct TerrainBindingRef(Uuid);
struct SkyEnvironmentBindingRef(Uuid);
struct RuntimeEntryRef(Uuid);
struct WorkspaceRef(Uuid);
struct ProfileRef(Uuid);
struct InteractionTargetRef(Uuid);
```

### Identity law
The editor-facing identity family is stable and opaque.
Frontend or scripting layers may serialize these ids as strings, but those strings are only an opaque transport form of the canonical stable ids.
Mixed local truths of `u32`, `number`, and ad hoc `string` ids are forbidden.

## Viewport ids
```rust
enum ViewportId {
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
    Custom(u32),
}
```

## Panel ids
```rust
enum PanelId {
    WorldOutliner,
    ContentBrowser,
    PropertyInspector,
    RuntimeInspector,
    DiagnosticsPanel,
    ConsolePanel,
    BuildPanel,
    ViewportStatusRail,
    Custom(String),
}
```

## Request and result classes
```rust
enum EditorRequestClass {
    Focus,
    Select,
    WorldOpen,
    WorldRestore,
    MutateEntity,
    Import,
    Validate,
    Preview,
    Build,
    RuntimeAttach,
    RuntimeDetach,
}

struct WorldOpenRequest { /* world_ref, open_mode, restore_policy */ }
struct WorldOpenResult { /* accepted, world_ref, failure_class */ }
struct WorldBindState { /* world_ref, terrain_ref, environment_ref, posture */ }
struct RuntimeEntryRequest { /* world_ref, mode, camera_policy */ }
struct RuntimeEntryResult { /* accepted, runtime_ref, deny_reason */ }
struct ReturnToAuthoringResult { /* accepted, world_ref, camera_policy, discarded_runtime_state */ }
```

## Mandatory viewport frame classes
```rust
struct ViewportBootFrame { /* world_ref, posture, placeholder_or_live_surface */ }
struct ViewportLiveFrame { /* world_ref, frame_ref, terrain_ref, environment_ref */ }
struct ViewportDegradedFrame { /* world_ref, degraded_reason, fidelity_class */ }
struct ViewportFailureProjection { /* failure_class, recovery_actions */ }
struct ViewportPlayAttachedFrame { /* runtime_ref, world_ref, camera_state */ }
struct ViewportSimulateAttachedFrame { /* runtime_ref, world_ref, camera_state */ }
struct ViewportDiagnosticsProjection { /* host, world, terrain, environment, runtime, budgets */ }
```

## Frozen semantic DTO families
The following semantic DTO families are mandatory and may not drift by host technology:

### `WorldSummaryDto`
- `worldRef`
- `worldLabel`
- `worldRole`
- `openMode`
- `bindPosture`
- `activeSelectionCount`

### `WorldOpenResultDto`
- `accepted`
- `worldRef`
- `worldLabel`
- `failureClass`
- `recoveryHints`

### `TerrainStateDto`
- `worldRef`
- `terrainBindingRef`
- `present`
- `walkable`
- `materialProfileRef`
- `lodPosture`
- `degraded`

### `SkyStateDto`
- `worldRef`
- `environmentBindingRef`
- `present`
- `timeOfDay`
- `dateOrCycleRef`
- `cloudPosture`
- `fogPosture`
- `precipitationPosture`
- `degraded`

### `ViewportFrameDto`
- `worldRef`
- `viewportId`
- `framePosture`
- `terrainPresent`
- `environmentPresent`
- `runtimeAttached`
- `failureClass`

### `ViewportStatsDto`
- `fps`
- `frameTimeMs`
- `budgetPosture`
- `terrainPosture`
- `environmentPosture`
- `runtimePosture`

### `PreviewStateDto`
- `sessionActive`
- `worldRef`
- `framePosture`
- `runtimeAttached`
- `degraded`

### `RuntimeEntryDto`
- `accepted`
- `runtimeRef`
- `mode`
- `worldRef`
- `denyReason`

### `DiagnosticsSummaryDto`
- `hostPosture`
- `worldPosture`
- `terrainPosture`
- `environmentPosture`
- `viewportPosture`
- `runtimePosture`
- `bridgePosture`
- `degradationPosture`

### `EntitySelectionDto`
- `worldRef`
- `entityId`
- `selectionSource`

### `EntityDetailsDto`
- `entityId`
- `label`
- `entityType`
- `transform`
- `layerRef`
- `componentSummaries`
- `assetBindings`

## DTO-shape law
For each DTO family there must be exactly one semantic field set.
Frontend mocks may mimic that field set.
They may not redefine it.
Backend adapters may only translate transport naming conventions.
They may not invent or drop semantic fields.

## Interaction and motion next-wave types
```rust
struct InteractionIntentId(Uuid);
struct ExecutionPolicyId(Uuid);
struct MotionAssetRef(Uuid);
struct ContactConstraintRef(Uuid);
struct InteractionDebugProjection { /* target, phase, weights, failure */ }
```

These types remain warm until the first world-open / terrain / environment / walk spine is honest.
