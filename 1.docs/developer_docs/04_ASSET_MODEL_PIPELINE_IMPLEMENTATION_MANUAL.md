# Asset/Model Pipeline Implementation Manual

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **developer guide**.


## Goal

Turn source assets into canonical runtime packages with validation, preview, binding, cook, and failure recovery.

## Implementation order

1. asset identity and source discovery;
2. format parser adapters;
3. normalized metadata;
4. validation verdicts;
5. preview artifact;
6. material slot binding;
7. collision generation/validation;
8. LOD/proxy/impostor generation;
9. runtime binding package;
10. cook package;
11. content browser status integration.

---

# V34 DCC source format, cooked asset, and Blender/3ds Max bridge closure

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Binding decision

StratumX supports DCC ecosystems through a layered import strategy:

1. **Direct exchange import** for open/common formats.
2. **DCC bridge import** for native `.blend` and `.max` workflows.
3. **Plugin adapter import** for renderer/material/CAD/add-on data that can be exported or inspected legally.
4. **External converter import** for niche formats where a stable converter exists.
5. **Manual quarantine** for unknown/proprietary data that cannot be validated.

No source DCC file is runtime truth. Runtime consumes only canonical cooked packages.

## Import support classes

| Class | Meaning | Runtime allowed? |
|---|---|---|
| P0 Direct | StratumX importer reads the file directly into canonical staging | yes after cook |
| P1 DCC Bridge | StratumX invokes Blender/3ds Max/headless script/export bridge | yes after cook |
| P2 Plugin Adapter | StratumX consumes exported plugin data or conversion report | yes after validation |
| P3 External Converter | StratumX consumes converter output with provenance | yes after validation |
| Q Quarantine | File recognized but not trusted or not supported | no |

## Blender ecosystem support

| Source | Extensions / families | Support class | Notes |
|---|---|---|---|
| Blender native scene | `.blend` | P1 bridge | Use Blender command/background export, not runtime direct parser. |
| glTF | `.glb`, `.gltf` | P0 | Preferred game interchange for mesh/material/animation where sufficient. |
| FBX | `.fbx` | P0/P1 | Accept as interchange; validate scale, axes, skeleton, materials. |
| USD | `.usd`, `.usda`, `.usdc`, `.usdz` | P0/P1 | Preferred for complex scene interchange when available. |
| Alembic | `.abc` | P0/P1 | Geometry/cache import; animation semantics limited by source. |
| OBJ/MTL | `.obj`, `.mtl` | P0 | Static geometry and simple materials. |
| PLY | `.ply` | P0 | Mesh/scan/point-derived geometry. |
| STL | `.stl` | P0 | Geometry only; no material truth without sidecar. |
| Collada legacy | `.dae` | P0/P1 | Legacy bridge; must validate transforms/materials. |
| BVH | `.bvh` | P0/P1 | Motion capture skeleton data; retargeting required. |
| SVG/vector | `.svg` | P0/P1 | UI/shape/source vectors; not runtime mesh until cooked. |
| Heightmaps/images | `.png`, `.tif`, `.exr`, `.hdr`, `.tga`, `.jpg`, `.webp` | P0 | Texture/terrain/source maps; color space verdict mandatory. |

## 3ds Max ecosystem support

| Source | Extensions / families | Support class | Notes |
|---|---|---|---|
| 3ds Max native | `.max` | P1 bridge | Requires 3ds Max installed or authorized converter; no runtime parser. |
| 3DS legacy | `.3ds` | P0/P1 | Legacy geometry/material; validate limitations. |
| FBX | `.fbx` | P0/P1 | Preferred 3ds Max game interchange. |
| USD | `.usd`, `.usda`, `.usdc`, `.usdz` | P0/P1 | Preferred scene interchange where Max pipeline exports it. |
| glTF | `.glb`, `.gltf` | P0/P1 | Good for runtime-oriented assets; validate material conversion. |
| OBJ/MTL | `.obj`, `.mtl` | P0 | Static mesh and simple materials. |
| Alembic | `.abc` | P0/P1 | Geometry cache. |
| CAD exchange | `.dwg`, `.dxf`, `.step`, `.stp`, `.iges`, `.igs`, `.sat`, `.skp`, Inventor/SolidWorks/CATIA via available importers/plugins | P1/P2/P3 | Import through 3ds Max or converter; generate real-time mesh/proxy/LOD. |
| Renderer plugins | V-Ray, Corona, Arnold, similar material/light/camera ecosystems | P2 | Convert to StratumX material/light profiles; unsupported nodes become diagnostic blockers. |
| Scatter/plugins | Forest Pack, RailClone, procedural placement ecosystems | P2 | Convert to placement fields, foliage archetypes, or baked instances with provenance. |

## Core runtime cooked formats

| Cooked asset | Purpose |
|---|---|
| `.sxmesh` | static mesh sections, bounds, material slots, LOD refs |
| `.sxskel` | skeleton, bind pose, retarget metadata |
| `.sxanim` | animation clip, root motion, contact markers |
| `.sxtex` | texture data, mips, compression, channel verdict |
| `.sxmat` | hybrid material profile, texture bindings, response refs |
| `.sxterrain` | height/layer/mask/collision tile |
| `.sxfoliage` | foliage archetype, placement field, wind class, impostor refs |
| `.sxgroom` | hair/fur coverage package, card/strand/shell tiers |
| `.sxcoll` | collision hull/mesh/traversal/breach class |
| `.sxaudio` | bank manifest, stream chunks, events, loudness |
| `.sxworldchunk` | cooked world cell/chunk entities, surfaces, fields |
| `.sxscene` | editor authoring scene manifest, not runtime truth by itself |

## Mandatory import route

```text
source file
  → source fingerprint
  → importer/bridge/converter selection
  → staging representation
  → coordinate/unit/axis normalization
  → geometry/material/skeleton/animation extraction
  → validation
  → canonical authoring asset
  → cook
  → runtime package
  → editor preview
  → diagnostics/evidence
```

## Blender bridge rules

The Blender bridge may run a headless script to export selected objects, collections, materials, animations, cameras, lights, geometry nodes bake results, and source metadata. The bridge must publish:

- Blender version;
- add-ons used;
- export format;
- unit scale;
- axis mapping;
- object count;
- material conversion report;
- unsupported modifier/add-on list;
- provenance hash.

## 3ds Max bridge rules

The 3ds Max bridge may use MaxScript/Python/batch export or authorized SDK tools. It must publish:

- 3ds Max version;
- plugin list relevant to the asset;
- export format;
- unit system;
- axis mapping;
- modifier stack collapse policy;
- material/light conversion report;
- CAD/procedural/scatter conversion report;
- unsupported plugin data list;
- provenance hash.

## Material conversion policy

DCC materials do not become StratumX materials directly. They become candidates for `.sxmat` hybrid profiles.

| DCC material feature | StratumX result |
|---|---|
| PBR base color/roughness/normal | direct profile channels |
| Spec/gloss workflow | converted to selected family model with diagnostic |
| V-Ray/Corona/Arnold nodes | converted if known, otherwise blocker with source map |
| Procedural texture | bake or unsupported blocker depending on policy |
| Multi/sub-object material | material slot table |
| Unknown node | fallback material + warning or hard blocker by profile |

## Geometry normalization

All imported model assets must receive:

- stable source asset id;
- scene object id map;
- normalized transform;
- unit scale verdict;
- axis conversion verdict;
- bounds;
- material slot map;
- tangent/normal policy;
- UV channel table;
- collision source/generation verdict;
- LOD/proxy/impostor verdict;
- provenance record.

## Acceptance

The asset pipeline is gold only when a developer can import:

1. Blender `.blend` through bridge;
2. Blender/Max `.fbx`;
3. `.glb/.gltf`;
4. `.obj/.mtl`;
5. `.usd` where toolchain supports it;
6. `.abc`;
7. heightmap/texture sources;
8. a 3ds Max `.max` scene through bridge;
9. one CAD-origin asset through 3ds Max/converter;
10. one plugin-material asset with a conversion report;

and each produces either a cooked package or an exact blocker with recovery instructions.
