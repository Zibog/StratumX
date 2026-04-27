# Terrain Material Texture Assignment And Layer Schema Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Give terrain material sets, texture stacks, cross-section visuals, and layer schemas one exact operator surface.

## Mandatory tabs
- layer schema
- material family binding
- texture stack
- visual variants
- streaming / LOD / cheapness
- cut and cross-section preview

## Mandatory controls
- assign terrain material set;
- bind layer-to-surface-family rows;
- bind albedo / normal / roughness / height / coverage families;
- bind cross-section family;
- preview wet / char / damage variants;
- inspect streaming class and visual LOD rung;
- capture before/after variant proof.

## Required overlays
- terrain layer mask overlay
- surface family overlay
- cross-section preview overlay
- residency pressure overlay

## Required inspector fields
- `terrain_material_set_ref`
- `terrain_layer_schema_ref`
- `cross_section_texture_ref`
- `visual_variant_bundle_ref`
- `material_visual_response_ref`
- `material_cheap_runtime_rung`
- `texture_streaming_class`

## Disabled reasons
`TRM_DISABLED_NO_WORLD`, `TRM_DISABLED_NO_TERRAIN_TARGET`, `TRM_DISABLED_LAYER_SCHEMA_LOCKED`, `TRM_DISABLED_TEXTURE_SET_INCOMPATIBLE`
