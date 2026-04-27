# World Conveyor Layered Authoring Workspace Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the editor as a layered world-building conveyor so operators can assemble complex simulation spaces without losing technology boundaries.

## Core posture
The editor shell must feel like one coherent creator conveyor.
The canonical world-building order is:
1. World overview
2. Terrain
3. Vegetation / biome
4. Structures / foundations / basements
5. Groups / assemblies / room sets / landscape sets
6. Materials forge
7. Audio surfaces and zones
8. Weather / sky / lighting
9. Simulation and validation
10. Build-ready review

This order is a UX contract.
It is not a truth-ownership collapse.
Underlying engine, sdk, and tooling ownership remains intact.

## Left-rail world conveyor law
The left rail must expose each domain step as a first-class shell item with:
- icon;
- human-readable name;
- progress or completeness badge;
- stale/blocked state;
- dependency hints;
- one-click open behavior.

Example dependency badges:
- structures depend on terrain fit and material/legality posture;
- vegetation depends on terrain, biome, and material families;
- audio depends on material, geometry, and zone placement;
- simulation review depends on valid world state and route closure.

## Domain workspace law
Each domain workspace must preserve the same shell skeleton:
- left rail persists;
- center stage changes to the relevant main surface;
- right inspector changes to the current domain truth scope;
- bottom strip keeps jobs, diagnostics, timeline, content, and assistant access.

This means creators do not re-learn the editor every time they cross into a new domain.

## Domain-specific center surfaces
### Terrain
Center stage may prioritize sculpt, layer stack, cross-section, and hydrology overlays.

### Vegetation
Center stage may prioritize biome rules, scatter paint, density masks, and explain-placement overlays.

### Structures
Center stage may prioritize placement, foundation fit, basement carve, support graph, and rupture preview.

### Groups
Center stage may prioritize set composition, anchors, placement scopes, variants, and scatter recipes.

### Materials
Center stage may prioritize layered family editing, response previews, where-used, and branch coverage.

### Audio
Center stage may prioritize zones, emitters, listener probes, occlusion paths, and bus consequences.

### Weather / Lighting
Center stage may prioritize cloud/front paths, sun/moon, flashlight proof, shadow diagnostics, and fog volumes.

### Simulation / Validation
Center stage may prioritize timeline scrubbing, incident cards, compare views, and blocker matrices.

## Structures and subterranean law
The workspace must visibly support the “sleeping layered world” posture.
This means the shell can author and inspect:
- terrain as layered substrate;
- structures fitted into that substrate rather than hovering over it;
- basement volumes embedded under existing terrain;
- wall rupture and soil spill consequences as lawful previews.

These capabilities may open specialized labs.
They must still read as natural parts of the same world conveyor.

## Grouping law
The editor must support grouped authoring not just single-asset placement.
Legal grouped concepts include:
- prop groups;
- room sets;
- building sets;
- dungeon sets;
- landscape sets;
- scatter recipes.

Grouped authoring must support scoped application to:
- selected room;
- selected building;
- terrain tile or region;
- underground volume;
- explicit selection set.

## Human-grade build-up law
The conveyor must help the operator think in world-making order while allowing expert jumping.
Therefore the shell must provide:
- quick-jump search;
- “return to previous domain” anchor;
- blockers that point to the exact upstream domain step;
- assistant suggestions expressed in conveyor terms rather than raw subsystem jargon.

## Prohibitions
- no shell where every domain opens an unrelated religion of UI;
- no terrain/material/audio/lighting fragmentation that destroys route memory;
- no stage-specific dead ends with no obvious upstream or downstream navigation;
- no one-game naming that would make the conveyor unusable for another simulation-heavy title.
