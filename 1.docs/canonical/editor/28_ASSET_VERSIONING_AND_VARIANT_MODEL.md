# Editor Asset Versioning and Variant Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Scope
The editor package consumes asset, prefab, scene-chunk, and package-versioning information from tooling but does not own asset authority.

## Asset identity posture
The editor may present:
- asset id and path
- source path
- version and import profile
- dependency lineage
- generated artifact sets
- platform/quality variants

## Prefab and authoring-variant posture
The editor must also support reusable authored objects with explicit lineage:
- prefab asset and prefab instance identity
- source version and instance GUID
- nested prefab relationships
- local override summaries
- override counts and modified property sets
- added/removed components and children
- local-only children
- variant-of relationships
- diff/apply/revert targets
- break-link / unpack posture

## Exact prefab actions
- Open Source
- Apply All
- Revert All
- Show Overrides
- Create Variant from Instance
- Unpack Completely
- Diff Local vs Source

## Versioning law
- identity remains stable across reimport or rebuild where legal
- version counters and lineage come from lower tooling
- local overrides are explicit authoring facts, never inferred by UI guesswork
- variant creation must remain traceable to explicit source lineage

## Exact prefab tools
- Create prefab from selection
- Open prefab in isolation
- Create variant
- Nest prefab
- Apply selected override
- Revert selected override
- Diff prefab
- Unpack
- Validate prefab contract
- Find all instances

## Exact borrowed prefab/package phrases
The canon preserves these exact borrowed phrases as implementation anchors:
- Nested Prefab
- Prefab Variant
- Prefab Editing Mode
- Package model
- Entity/Prefab workflows
- Procedural Prefab workflows
