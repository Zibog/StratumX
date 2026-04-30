# Asset/Model Schema Reference

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **API reference**.

---

# V34 DCC and cooked asset schema closure

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Source asset descriptor

| Field | Type | Meaning |
|---|---|---|
| `source_asset_id` | stable id | Identity of the source file/import root. |
| `source_path` | path | Original source path or virtual package path. |
| `source_format` | enum | blend, max, fbx, gltf, glb, usd, obj, abc, dae, ply, stl, image, cad, audio, unknown. |
| `support_class` | enum | P0, P1, P2, P3, Q. |
| `dcc_bridge` | optional enum | blender, 3ds_max, external_converter. |
| `source_fingerprint` | hash | File/content/provenance fingerprint. |
| `unit_scale` | float | Source to meters. |
| `axis_mapping` | enum | source to StratumX orientation. |
| `plugin_manifest` | list | DCC/plugin/add-on evidence. |

## Cooked asset descriptor

| Field | Type | Meaning |
|---|---|---|
| `cooked_asset_id` | stable id | Runtime package identity. |
| `cooked_family` | enum | sxmesh, sxskel, sxanim, sxtex, sxmat, sxterrain, sxfoliage, sxgroom, sxcoll, sxaudio, sxworldchunk. |
| `source_asset_id` | ref | Source provenance. |
| `validation_verdict` | enum | pass, warning, degraded, blocker. |
| `runtime_ready` | bool | True only when package is legal for runtime. |
| `dependencies` | refs | Textures, materials, collisions, skeletons, banks, world chunks. |
| `evidence_id` | ref | Import/cook proof artifact. |

## Import failure codes

- `asset.import.unsupported_format`
- `asset.import.dcc_bridge_missing`
- `asset.import.plugin_data_unsupported`
- `asset.import.material_conversion_blocked`
- `asset.import.unit_axis_unresolved`
- `asset.import.missing_texture`
- `asset.import.invalid_skeleton`
- `asset.import.no_collision_policy`
- `asset.cook.runtime_package_failed`
- `asset.reimport.identity_conflict`
