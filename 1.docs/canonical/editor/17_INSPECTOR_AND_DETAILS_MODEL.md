# Inspector and Details Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Responsibilities
- typed details for focused objects and selections
- batch editing where legal
- validation hints, provenance, and diff display
- preview-safe value staging
- assistant and diagnostics context handoff
- prefab and override inspection
- runtime vs authoring comparison where legal
- terrain and sky/environment binding inspection where relevant

## Required fixed header fields
- Name
- Stable ID
- Prefab Root ID
- Source Asset
- Tags
- Layer
- Data Layer
- Mobility = Static / Stationary / Dynamic
- Streaming Policy = Always Loaded / Cell Loaded / Distance Loaded / Manual
- Runtime State = Authoring / Preview / Simulating / Broken
- Override State = Clean / Has Local Overrides / Diverged
- Validation State = Pass / Warning / Error
- Owner Package
- Last Modified By Tool

## Required header actions
- Apply Overrides
- Revert Overrides
- Open Source Prefab
- Diff vs Source
- Validate
- Bake
- Pin
- Copy Component Path
- Inspect Bindings

## Terrain/environment bind section
When the selected object is a world, terrain root, or environment root, the inspector must expose:
- Terrain Binding Ref
- Sky Environment Binding Ref
- active profile refs
- bind state
- validation state
- diagnostics link-outs

The inspector may stage edits and requests.
It may not own terrain or sky truth.

## Laws
- inspector owns presentation and staging only
- staged edits must lower into legal lower-stack requests
- hidden persistent side stores are forbidden
- prefab apply/revert and component edits must remain transaction-visible and diffable
- bind sections must explain whether they are world-backed, preview-backed, degraded, or missing

## Exact borrowed inspector labels
The property surface is intentionally searchable under the exact aliases:
- Details / Inspector
- Entity Inspector
