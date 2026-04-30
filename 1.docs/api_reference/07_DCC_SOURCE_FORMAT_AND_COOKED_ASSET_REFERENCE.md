# DCC Source Format and Cooked Asset Reference

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **API/reference document**.

## Support class enum

| Value | Meaning |
|---|---|
| `P0_DIRECT` | StratumX direct importer reads source file. |
| `P1_DCC_BRIDGE` | DCC application bridge exports/stages the data. |
| `P2_PLUGIN_ADAPTER` | Plugin/add-on data is converted through adapter report. |
| `P3_EXTERNAL_CONVERTER` | External converter output is accepted with provenance. |
| `Q_QUARANTINE` | Recognized but not legal for runtime/cook. |

## Format enum

| Enum | Extensions |
|---|---|
| `BLENDER_BLEND` | `.blend` |
| `AUTODESK_MAX` | `.max` |
| `FBX` | `.fbx` |
| `GLTF` | `.gltf`, `.glb` |
| `USD` | `.usd`, `.usda`, `.usdc`, `.usdz` |
| `OBJ` | `.obj`, `.mtl` |
| `ALEMBIC` | `.abc` |
| `COLLADA` | `.dae` |
| `PLY` | `.ply` |
| `STL` | `.stl` |
| `BVH` | `.bvh` |
| `CAD_EXCHANGE` | `.dwg`, `.dxf`, `.step`, `.stp`, `.iges`, `.igs`, `.sat`, `.skp`, vendor-specific via adapter |
| `IMAGE_TEXTURE` | `.png`, `.tga`, `.tif`, `.tiff`, `.exr`, `.hdr`, `.jpg`, `.jpeg`, `.webp` |
| `AUDIO_SOURCE` | `.wav`, `.flac`, `.ogg` where backend supports decode |

## Cooked format enum

| Enum | Extension |
|---|---|
| `SX_MESH` | `.sxmesh` |
| `SX_SKELETON` | `.sxskel` |
| `SX_ANIMATION` | `.sxanim` |
| `SX_TEXTURE` | `.sxtex` |
| `SX_MATERIAL` | `.sxmat` |
| `SX_TERRAIN` | `.sxterrain` |
| `SX_FOLIAGE` | `.sxfoliage` |
| `SX_GROOM` | `.sxgroom` |
| `SX_COLLISION` | `.sxcoll` |
| `SX_AUDIO_BANK` | `.sxaudio` |
| `SX_WORLD_CHUNK` | `.sxworldchunk` |

## Required diagnostic fields

- importer route;
- support class;
- source DCC version;
- source plugin list;
- unsupported feature list;
- conversion warnings;
- blocker codes;
- cooked package ids;
- evidence artifact id.
