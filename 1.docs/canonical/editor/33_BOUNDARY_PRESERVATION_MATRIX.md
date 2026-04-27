# Boundary Preservation Matrix

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Editor vs Tooling Boundary

| Editor Surface | Legal Tooling Dependencies | Tooling-owned truth / runtime |
|----------------|----------------------------|-------------------------------|
| L8.0 Editor Shell | `L6.7 stream_plane`, `L6.9 budget_runtime`, `L6.10 workspace_runtime`, `L6A`, `L7` status surfaces | lower-runtime status, session-safe refs only |
| L8.1 Viewport | `L6.0 authority_core`, `L6.3 snapshot_plane`, `L6.5 derived_plane`, `L6.12 preview_runtime` | world truth projections and previews |
| L8.2 Outliner | `L6.0 authority_core`, `L6.4 index_plane`, `L6.5 derived_plane` | hierarchy, region, layer, and runtime-loaded projections |
| L8.3 Content Browser | `L6.4 index_plane`, `L6.5 derived_plane`, `L6.6 artifact_plane`, `L6.13 build_runtime`, `L6.14 release_runtime` | asset/dependency/build/release projections |
| L8.4 Inspector | `L6.3 snapshot_plane`, `L6.5 derived_plane`, `L6.11 validation_runtime`, `L6.12 preview_runtime` | details, diffs, validation, preview results |
| L8.5-L8.8 Interaction systems | `L6.1 command_envelopes`, `L6.2 transaction_ledger`, `L6.10 workspace_runtime` | legal request lowering and public ref coordination |
| L8.9 Assistant Surface | `L6A` assistant runtime | proposal/evidence/apply-revert runtime |
| L8.10 Diagnostics Surface | `L6.7 stream_plane`, `L6.11 validation_runtime`, `L6.13 build_runtime`, `L6.14 release_runtime` | diagnostics and queue state |
| L8.11 Build/Release Surface | `L6.11 validation_runtime`, `L6.13 build_runtime`, `L6.14 release_runtime`, `L7` reporting/governance where needed | validation/build/release truth |
| L9 Suites | `L6.0-L6.14` as required through public surfaces only | authoring truth remains below |
| L10.1 Import/Export | `L6.6 artifact_plane`, `L6.11 validation_runtime`, `L6.13 build_runtime`, `L6.14 release_runtime` | asset processor, bake, build, export truth |
| L10.5 Plugin Host | `L6.1`, `L6.2`, `L6.11`, `L6.12`, `L6.13`, `L6.14` through public APIs | plugins do not own authority |
| L10.7 Package/Dependency | `L6.4 index_plane`, `L6.5 derived_plane`, `L6.6 artifact_plane`, `L6.11`, `L6.13`, `L6.14` | dependency and package projections |
| L11.3 Playtest/Capture | `L6.7 stream_plane`, `L6.11`, `L6.12`, `L6.13` | runtime attach and capture truth |

## Editor vs SDK Boundary

| Editor Surface | Consumes from SDK | Must NOT Own |
|----------------|-------------------|--------------|
| All Surfaces | windowing/input/math/serialization primitives | tool/runtime truth beyond UI-local state |
| Serialization Helpers | `L4` serialization-and-io | lower-runtime manifests or package truth |

## Editor vs Engine Boundary

| Editor Surface | Consumes from Engine | Must NOT Own |
|----------------|----------------------|--------------|
| All Surfaces | NONE (must flow through tooling) | engine truth, engine authority |

The editor must NEVER consume engine surfaces directly. All engine consumption must flow through tooling package (`L6/L6A/L7/L7A`).

## Boundary Preservation Rules

### Read Discipline
- all world/entity/prefab/layer truth reads flow through `snapshot/index/derived`
- all artifact/build/release truth reads flow through `artifact/build/release`
- all validation reads flow through `validation_runtime`
- all preview reads flow through `preview_runtime`
- all assistant reads flow through `L6A`
- all reporting/governance reads flow through bounded `L7` surfaces

### Write Discipline
- all committed truth writes flow through `command_envelopes -> transaction_ledger`
- all preview writes are requests into `preview_runtime`
- all validation writes are requests into `validation_runtime`
- all import/reimport/bake/build writes are requests into `build_runtime`
- all release/package writes are requests into `release_runtime`

### Ownership Discipline
- editor owns product-local UI state only
- tooling owns lower-runtime truth, queues, manifests, and bounded streams
- sdk owns bridge types and facts only
- engine owns engine runtime only
- no layer may maintain shadow truth or parallel state
