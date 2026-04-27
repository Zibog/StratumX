# Editor Dataflow and Activation Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

The editor consumes lower-stack immutable results, indices, queues, cursors, projections, and bounded streams.
It never becomes the hidden owner of authoritative runtime or world data.

## Primary data classes consumed by the product
- world open and bind projections
- viewport frame projections
- terrain and environment binding projections
- selection and inspector projections
- validation and diagnostics projections
- runtime-entry and runtime-watch projections
- build and release queue projections

## Canonical top-level path
The canonical path from engine to visible frame is:
`engine truth -> sdk bridge packets/observations -> tooling preview/runtime/session projections -> editor viewport/panel surfaces -> visible frame and controls`.

## Canonical command path
The canonical path from a button, menu item, or shortcut is:
`editor intent -> tooling command envelope -> tooling transaction ledger / runtime coordinator -> sdk ingress -> engine / startup / runtime / synthesis -> sdk egress -> tooling projection -> editor refresh`.

## Activation posture
### Hot
- primary viewport
- active world-open workflow
- active selection and inspector
- active diagnostics rail
- active play/simulate controls
- terrain and environment suites when the world is open

### Warm
- outliner and content browser
- validation summary and runtime inspector
- animation/cinematics debug surfaces

### Cold
- unopened suites
- secondary viewports
- graph-heavy tools not involved in the current closure thread

## First closure dataflow blocks
### 1. Host and workspace block
- one canonical host launches;
- workspace restore state is resolved;
- editor shell activates the primary surface set;
- diagnostics publish host-selection posture.

### 2. World-open block
- editor issues `WorldOpenRequest`;
- tooling validates workspace/session legality;
- sdk bridges request;
- engine/startup resolves `StartupWorldRef` or explicit open target;
- tooling publishes `WorldOpenResult` and `WorldBindState`;
- editor surfaces world identity and failure posture.

### 3. Terrain presentation block
- engine world/imaging publishes terrain-facing state;
- sdk emits terrain-safe observations;
- tooling builds terrain projections;
- editor viewport presents terrain and terrain diagnostics.

### 4. Sky/environment block
- world binding includes `SkyEnvironmentBindingRef` where legal;
- engine field/imaging/startup resolves environment state;
- sdk emits environment observations;
- tooling composes environment projections;
- editor viewport and diagnostics present them without owning the truth.

### 5. Frame composition block
- tooling merges world, terrain, environment, overlays, and diagnostics into one viewport frame contract;
- editor draws one authoritative frame state plus optional presentation overlays;
- the frame carries posture class: boot, live, degraded, failure, play-attached, or simulate-attached.

### 6. Runtime-entry block
- editor issues `RuntimeEntryRequest`;
- tooling checks world-open, bind, diagnostics, and capability posture;
- sdk bridges request;
- engine runtime enters play/simulate or denies entry;
- tooling emits `RuntimeEntryResult` and `PlayWalkStateObservation`;
- editor updates controls and viewport posture.

### 7. Return-to-authoring block
- editor issues return request;
- tooling coordinates runtime detach;
- sdk publishes return result;
- tooling restores authoring projections;
- editor restores selection, camera, and diagnostics by explicit policy.

## Law
The editor may cache view state, interaction context, and ephemeral layout state.
It may not invent world, terrain, environment, frame, or runtime outcomes without lower-stack evidence.
