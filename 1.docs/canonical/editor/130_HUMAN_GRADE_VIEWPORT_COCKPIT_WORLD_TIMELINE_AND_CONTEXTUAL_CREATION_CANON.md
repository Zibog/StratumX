# Human Grade Viewport Cockpit World Timeline And Contextual Creation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the viewport as a dense but humane cockpit so the editor feels alive, explainable, and productive from the first open world.

## Primary law
The viewport is the emotional and operational center of the editor.
It must feel immediately useful even in an empty or early project.
It may not wait for ten other windows before becoming informative.

## Cockpit frame
The canonical viewport cockpit contains:
- top viewport toolbar;
- left contextual creation bar;
- top-right world status cluster;
- right quick-inspector ribbon;
- bottom world timeline and event strip;
- overlay selector and probe selector;
- optional embedded minimap/constellation entry point.

## Top toolbar law
The toolbar must provide rapid access to:
- transform posture;
- camera posture;
- snapping and pivot posture;
- overlay families;
- split view;
- render/debug presentation modes;
- selection filters;
- capture readiness and compare eligibility.

## Right quick-inspector ribbon
The ribbon exists for hot values that should not require full panel travel.
Examples:
- world position / region / chunk id;
- terrain layer under cursor;
- material family under cursor;
- wetness / temperature / structural stress sample;
- current listener/audibility sample;
- current AI zone or causality hint.

The ribbon is observational only unless a lawful quick action is explicitly routed.

## Bottom world timeline and event strip
The strip must show recent or current world events such as:
- terrain edits;
- vegetation or group placements;
- structure fit/basement operations;
- weather transitions;
- fire/wetness/smoke incidents;
- collapse or deformation incidents;
- validation failures;
- bake and job completion states.

Each event card must support:
- focus in viewport;
- open owning lab or inspector;
- reveal blocker or route lineage;
- bookmark for later compare/capture review.

## Contextual creation bar
The left creation bar is the main source of “it feels like a creator tool”.
It must provide the primary create/paint/sample/probe actions for the active domain.
It must change with stage while preserving position, density, icon style, and interaction patterns.

### Examples of lawful actions by stage
#### Terrain
- sculpt
- flatten
- smooth
- cut trench / basement
- stamp crater or feature
- paint layer
- sample substrate

#### Vegetation
- paint species
- scatter species
- erase cluster
- randomize age
- explain placement

#### Structures
- place structure
- snap to terrain
- embed foundation
- carve basement
- preview soil displacement
- preview rupture spill

#### Groups
- create group from selection
- stamp group
- scatter group
- conform to room/building/landscape
- validate intersections

#### Materials
- sample material
- preview contact/bullet/blast/wetness/burn
- inspect where-used
- compare variant

#### Audio
- place emitter
- paint zone
- run listener probe
- preview occlusion
- preview indoor/outdoor transition

## Empty-state law
An empty viewport must still be useful.
It must show:
- project/world identity;
- the next recommended conveyor step;
- current stage;
- available lawful creation actions;
- one clear path to begin authoring.

It may not show a decorative nothingness card that teaches the user nothing.

## Diagnostics-in-place law
The viewport must explain important failures in-place.
Examples:
- asset missing;
- world validation blocker;
- render chain unavailable;
- stale bake;
- denied stage action.

Failure explanation must support direct recovery focus.

## Relationship to other shell bands
This document consumes and extends:
- `114` shell workspace law;
- `115` viewport honesty law;
- `117` stage-strip law;
- `128` visual language;
- `129` world conveyor law;
- `131` constellation navigator law.
