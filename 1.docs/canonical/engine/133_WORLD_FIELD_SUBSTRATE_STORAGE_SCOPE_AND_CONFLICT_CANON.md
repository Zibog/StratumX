# World Field Substrate Storage Scope And Conflict Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the storage, update, and conflict law for the shared field substrate used by wetness, heat, smoke, contamination, wind, obscuration, sound hints, and anomaly carriers.

## Exact truth objects
- `CellFieldAtlas`
- `SurfaceFieldSlice`
- `ObjectLocalFieldBinding`
- `VolumeFieldBrickSet`
- `FieldWriteIntent`
- `FieldConflictVerdict`

## Scope law
- cell scope is authoritative for world propagation;
- surface scope is authoritative for material/terrain modulation;
- object-local scope is authoritative only inside one host envelope;
- volume scope is authoritative for airborne or bounded 3D carriers.

## Update order
`ingest -> accumulate -> diffuse/advect -> resolve conflicts -> publish consumers -> seal replay`

## Conflict precedence
`explicit source truth -> substrate carrier -> derived gameplay aids -> visual/audio-only hints`

## Forbidden shortcuts
- direct domain-to-domain writes that bypass the field substrate when a canonical field family exists;
- silent promotion of debug overlays into authoritative field writes;
- local ad-hoc volume grids disconnected from cell identity.

## Failure families
- `world.field.scope_illegal`
- `world.field.conflict_unresolved`
- `world.field.cell_surface_desync`
- `world.field.replay_gap`

## Current posture
`document_gold / substrate_runtime_closed / implementation_open`
