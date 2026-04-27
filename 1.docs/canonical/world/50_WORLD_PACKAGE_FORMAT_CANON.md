# World Package Format Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the canonical world package format for disk storage, loading, identity preservation, terrain-layer truth, and material-first world authoring.

## Canonical World Package Layout

```
my_world/
  world.json                    # Manifest (required)
  terrain/
    terrain_manifest.json       # Terrain configuration
    chunks/
      chunk_0_0.bin            # Height + layer-weight data per chunk
      chunk_0_1.bin
  environment/
    sky_binding.json           # Sky/weather configuration
  materials/
    material_registry.json     # Material archetype / surface-family bindings used by this world
    instances/
      instance_0001.json       # Optional material instance stacks when world-local
  content/
    placements.json            # Entity placements
  diagnostics/
    baselines/                 # Validation baselines
```

## world.json Schema (Required)

```json
{
  "world_id": "uuid-string",
  "world_label": "My World",
  "world_role": "startup|reference|content",
  "version": "1.0.0",
  "terrain_root_ref": "terrain/terrain_manifest.json",
  "environment_root_ref": "environment/sky_binding.json",
  "material_registry_ref": "materials/material_registry.json",
  "streaming_profile_ref": null,
  "source_lineage": {
    "created_at": "2026-04-06T12:00:00Z",
    "created_by": "editor",
    "import_source": null,
    "last_modified": "2026-04-06T12:00:00Z"
  }
}
```

## terrain_manifest.json Schema

```json
{
  "terrain_id": "uuid-string",
  "origin": [0.0, 0.0, 0.0],
  "world_size": [1000.0, 1000.0],
  "resolution": [512, 512],
  "chunk_grid": [4, 4],
  "chunk_size": 256,
  "height_range": [-100.0, 500.0],
  "material_layers": [
    {
      "layer_id": 0,
      "surface_family_ref": "surface.terrain.grass_field",
      "material_archetype_ref": "archetype.grass_cover",
      "response_profile_ref": "response.terrain.grass_default",
      "blend_policy_ref": "blend.terrain.soft_weighted"
    }
  ],
  "chunks": [
    {
      "chunk_x": 0,
      "chunk_y": 0,
      "data_file": "chunks/chunk_0_0.bin",
      "revision": 1
    }
  ]
}
```

## material_registry.json Schema

```json
{
  "registry_id": "uuid-string",
  "surface_bindings": [
    {
      "surface_family_ref": "surface.terrain.asphalt_road",
      "material_archetype_ref": "archetype.asphalt_road",
      "response_profile_ref": "response.terrain.asphalt_default"
    }
  ],
  "world_local_instances": [
    "instances/instance_0001.json"
  ]
}
```

## sky_binding.json Schema

```json
{
  "sky_bundle_ref": "shared/sky/sky_bundle.json",
  "time_of_day": 12.0,
  "day_of_year": 180,
  "latitude_deg": 45.0,
  "weather_regime": "Clear",
  "cloud_coverage": 0.3,
  "fog_density": 0.0
}
```

## Chunk Data Format (.bin)

Binary format:
- Header: 16 bytes
  - Magic: "SXCH" (4 bytes)
  - Version: u32 (4 bytes)
  - Width: u32 (4 bytes)
  - Height: u32 (4 bytes)
- Height data: f32 array (width * height * 4 bytes)
- Material weights: [f32; 4] array (width * height * 16 bytes)
- Optional aftermath overlay weights: [f32; 4] array when aftermath serialization is enabled for the chunk

## Loading Rules

1. **world.json is mandatory** - without it, path is not a valid world
2. **world_id must be stable** - same world = same ID across sessions
3. **terrain_root_ref is optional** - world can exist without terrain
4. **environment_root_ref is optional** - world can exist without sky
5. **material_registry_ref is optional only when every material ref resolves to a shared canonical registry**
6. **Missing refs = controlled degradation** - not failure

## Validation Rules

1. Check world.json exists and parses
2. Validate world_id is valid UUID
3. Check referenced manifests exist if refs are present
4. Validate chunk data files exist if listed in manifest
5. Validate every material layer resolves one surface family and one archetype
6. Report missing assets as degraded posture, not failure

## Identity Preservation

- `world_id` is the stable identity
- `source_lineage` tracks creation and modification
- `version` allows migration between formats
- `world_role` distinguishes startup/reference/content
- world save may not invent new material or terrain identities silently

## Failure Postures

- **Missing world.json**: `WorldNotFound`
- **Invalid JSON**: `CorruptedData`
- **Missing world_id**: `CorruptedData`
- **Missing terrain manifest**: Degraded (terrain disabled)
- **Missing environment manifest**: Degraded (environment disabled)
- **Missing material registry**: Degraded only if all refs still resolve through shared canon
- **Missing chunk data**: Degraded (holes in terrain)

## Canonical Truth

The world package on disk is the source of truth.
Editor modifications must write back to the package.
No silent in-memory-only changes.


## Companion world-family docs
- `51_WORLD_IDENTITY_AND_NAMESPACE_CANON.md`
- `52_WORLD_CHUNK_INVALIDATION_AND_REBUILD_CANON.md`
- `53_WORLD_STREAMING_PROFILE_AND_RESIDENCY_CANON.md`
- `54_WORLD_MATERIAL_REGISTRY_LINKAGE_CANON.md`
- `55_WORLD_PERSISTENCE_AND_SAVE_RESTORE_CANON.md`
- `56_WORLD_VALIDATION_AND_DEGRADED_POSTURE_CANON.md`


## World family closure note
This package spec is not alone.
It is read together with `world/51–56` for identity, rebuild, streaming, material linkage, save/restore, and validation law.
A world package is not gold if those companion laws are bypassed.

## Material language unification note
Terrain, prop, structure, and foliage surfaces all resolve through one world-readable material vocabulary:
- `surface_family_ref`
- `material_archetype_ref`
- `response_profile_ref`
- optional local material instance stack
- optional biome / aftermath overlays

## Material-centric storage restriction
World packages may store canonical material-facing truth only:
- archetype refs
- surface family refs
- normalized response family refs
- lawful state modifiers
- cheap-runtime posture refs when persistence is declared
- aftermath and overlay refs that world law says must survive save/restore

World packages may not store branch-private transient noise, renderer-only aliases, or audio-only substitute ids as durable truth.


## Material proof and persistence note
World storage must remain consistent with root `94` and `95`.
Persistent world data may not encode proof-only artifacts as durable truth, but it must preserve any material-facing persistence families and aftermath refs that those registries declare world-retainable.
