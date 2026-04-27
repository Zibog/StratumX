# Outliner and World Browser Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Responsibilities
- hierarchy projection
- world partition and region views
- data-layer and layer visibility/load views
- visibility, lock, active, and dirty toggles
- grouping, parenting, and folder views
- search, filter, and scope slicing
- terrain and sky/environment bind visibility for the opened world

## Required outliner columns
- Visible
- Selectable
- Active
- Dirty
- Lock
- Name
- Type
- Layer
- Prefab/Scene Source
- Transform Modified
- Owner
- Runtime Loaded
- GUID

## Required context actions
- Create Empty
- Create From Prefab
- Convert Selection to Prefab
- Unpack Prefab
- Duplicate
- Delete
- Rename
- Move to Layer
- Assign Data Layer
- Save as Scene Chunk
- Reveal in Content Browser
- Pin in Inspector
- Copy GUID
- Copy Path
- Mark Static / Dynamic
- Lock Editing
- Create Terrain Root
- Bind Terrain
- Create Sky Environment
- Bind Sky Environment

## Coherence law
Outliner selection, inspector focus, viewport highlight, and diagnostics references must resolve to the same entity identity family.
The outliner may not invent alternate string-only ids or alternate numeric-only ids once the shared type registry has frozen the canonical identity model.

## Laws
- hierarchy views are projections of lower-stack state
- grouping actions become legal lower-stack requests
- world browser hosts scale to very large scenes through activation-bounded indexing and partial projection
- the outliner may expose data-layer/runtime-loaded posture, but may not own it

## Exact borrowed world-browser labels
The world hierarchy surface is intentionally searchable under the exact aliases:
- World Outliner / Scene Tree
- Scene Tree
- World Partition
- Level Instancing
- One File Per Actor / External Actors

## Exact world tool labels
- Cell Load/Unload
- World chunk save
- HLOD bake trigger
- Navigation rebuild region
- Reference graph by region
