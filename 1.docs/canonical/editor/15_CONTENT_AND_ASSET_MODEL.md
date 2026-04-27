# Content and Asset Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Asset classes
- source assets
- imported/intermediate artifacts
- authored project assets
- preview artifacts
- baked/cooked/build artifacts
- release artifacts

## Product responsibilities
- browse and filter assets
- inspect metadata and health
- stage imports, reimports, conversions, rebuilds, and exports
- show dependencies, reverse dependencies, references, and reverse references
- surface asset-processor, bake, build, and release status
- expose bundle/addressable/streamable posture where legal

## Required content-browser columns
- Name
- Type
- Source File
- Imported Hash
- Artifact Hash
- Status
- Labels
- Size
- Dependencies
- Reverse Dependencies
- Last Import Time
- Cooker/Baker Status

## Required filters
- Meshes
- Materials
- Textures
- Animations
- Sounds
- VFX
- Prefabs
- Scenes
- Blueprints/Graphs
- Scripts
- Data Assets
- Broken Assets
- Unused Assets
- Dirty Assets

## Required content actions
- Reimport
- Rebuild Artifacts
- Validate Dependencies
- Find References
- Find Reverse References
- Create Variant
- Create Prefab
- Open Raw Source
- Reveal Generated Files
- Mark Addressable / Streamable
- Assign to Bundle

## Asset-processor and import posture
The editor must surface exact asset-processor state for importable content:
- watch filesystem
- queue imports
- build artifacts
- cache build outputs
- dependency tracking
- invalidate on source change
- report errors back to UI

## Laws
- source and imported artifact classes remain distinct
- asset browsing views are read-mostly and index-backed
- import, conversion, bake, and build work belongs to lower services, not shell views
- broken, unused, dirty, and quarantined states must be explicit and searchable

## Exact borrowed asset-browser labels
The content surface is intentionally searchable under the exact aliases:
- Content Browser / FileSystem
- FileSystem Dock

## Exact package/build labels surfaced here or through linked panels
- Package Manager
- Build Status


## Exact asset import field labels
- asset_id
- source_path
- importer_type
- import_profile
- last_import_hash
- generated_artifact_ids[]
- dependency_ids[]
- platform_targets[]
- compression
- cook_rule
- streaming_group
- status
- warnings_count
- errors_count

### Exact mesh import settings
- Import Normals
- Recalculate Tangents
- Generate LODs
- Collision Generation
- Mesh Scale
- Pivot Rule
- Material Slot Mapping
- Build Navigation Geometry
- Allow Runtime Instancing

### Exact texture import settings
- sRGB
- Compression Preset
- Mip Generation
- Max Resolution
- Address Mode
- Normal Map
- Virtual/Streamed
- Alpha Mode

### Exact animation import settings
- Skeleton Binding
- Root Motion
- Compression
- Loop
- Events
- Retarget Profile

## Required first-closure worldspace asset families
For the first product-result worldspace closure, the content model must cleanly distinguish at least the following authored families:
- world assets and startup-world refs;
- landscape/terrain source assets;
- landscape layer sets and paint/sculpt data families;
- terrain-surface material/profile assets;
- sky/environment profile assets (`Sky`, `Atmosphere`, `Cloud`, `Weather`, `Fog`);
- world-to-environment binding refs;
- player/walk-start or equivalent runtime-entry refs for the first traversal loop;
- viewport capture presets used for proof and diagnostics.

The editor may aggregate browsing of these families.
It may not collapse them into unnamed generic blobs.
