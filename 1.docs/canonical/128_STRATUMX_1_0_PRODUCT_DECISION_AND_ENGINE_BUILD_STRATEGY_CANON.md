# StratumX 1.0 Product Decision and Engine Build Strategy Canon

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **root canonical decision ledger**.

This document turns the open design questions into binding StratumX 1.0 decisions. It exists so engine, SDK, tooling, editor, documentation, and future implementation agents stop expanding the dream stack sideways and instead build along fixed rails.

## 0. Non-negotiable meaning of StratumX 1.0

StratumX 1.0 is **not** the whole dream game and is not the full Chernobyl Exclusion Zone at final fidelity.

StratumX 1.0 is:

> An editor-first, material-first, large-world-capable engine foundation with a Native Graphics Port, canonical asset pipeline, hybrid material model, lawful SDK/tooling bridge, diagnostics/capture/evidence contour, and one first proof region that can be expanded into the dream-stack without rewriting the engine.

## 1. Official 1.0 victory lane

The minimum StratumX 1.0 victory is:

| Step | Required result | Must not be fake |
|---|---|---|
| Empty project | Project opens through editor bootstrap | No hidden sample-only path |
| Graphics backend | Native Graphics Port selects backend or null | No Vulkan-centric engine truth |
| Showable frame | Terrain + sky + material + light + capture | No egui-only fake viewport |
| Asset import | DCC source becomes canonical cooked asset | No direct runtime source-file dependency |
| Material assignment | Hybrid material profile binds visual and future response channels | No shader-only material truth |
| World placement | Imported object placed into cell/chunk/world hierarchy | No loose scene soup |
| Diagnostics | First blocker, disabled reason, capture artifact visible | No silent failure |
| Save/load | Project and world restore without manual rebuild | No temporary memory-only success |

## 2. The twenty binding decisions

| # | Decision | Binding answer |
|---:|---|---|
| 1 | What is StratumX 1.0? | Editor-first engine foundation, not full dream-stack. |
| 2 | First vertical slice | Empty project → showable frame → asset import → material assignment → capture. |
| 3 | Truth vs view | Engine owns truth; SDK carries packets; tooling executes routes; editor edits/views; apps launch only. |
| 4 | Graphics architecture | StratumX Native Graphics Port owns the graphics contract; no native API is canonical truth. |
| 5 | First graphics backend | Null is mandatory first validation backend; Vulkan is first real backend; DX12/Metal/platform-native are first-class stubs. |
| 6 | Material model | Hybrid: profile-driven runtime families now; optional node graph later; graph never bypasses profiles. |
| 7 | Asset model | Source assets are never runtime truth; all runtime assets are canonical cooked packages. |
| 8 | DCC support | Blender and 3ds Max are first-class source ecosystems through direct import, DCC bridge, plugin adapter, or external converter. |
| 9 | World format | World → Region → Sector → Cell → Chunk → Entity/Surface/Field. Authoring uses stable global coordinates; runtime uses local cell coordinates. |
| 10 | Entity model | Hybrid entity-component: stable WorldEntity identity, components as data, systems as behavior, editor object as authoring projection. |
| 11 | Scheduler | Engine owns phase scheduler/task graph; domains do not spawn arbitrary global threads. |
| 12 | Physics strategy | Replaceable physics kernel adapter below StratumX material response, topology, and field substrate. |
| 13 | Audio strategy | StratumX owns event/mix/material sound truth; audio device/mixer backend is replaceable. |
| 14 | Netcode target | 1.0 is single-player/local deterministic foundation; 1.x adds server-authoritative multiplayer foundation. |
| 15 | Replication model | Replicate intents/events/state deltas and retained summaries, not the entire world every frame. |
| 16 | Scripting | 1.0 has no arbitrary gameplay scripting in core; data-driven commands first; sandboxed plugin/scripting later. |
| 17 | Plugin architecture | Plugins may contribute panels, commands, importers, validators, previews, assistant skills; they cannot bypass truth. |
| 18 | Documentation split | canonical = law; developer_docs = implementation; user_docs = editor use; api_reference = packets/commands; tutorials = walkthrough; troubleshooting = recovery. |
| 19 | Quality gates | Workspace, layer deps, file size, test placement, doc marker, orphan/stub, evidence gates are mandatory. |
| 20 | Dream-scene staging | Build beautiful frame first, then material visual truth, physical preview, weather, hydrology/fire, AI/living, proof region, large-world expansion. |

## 3. Official staging ladder

| Stage | Name | Result |
|---:|---|---|
| S1 | Beautiful frame | Terrain + sky + one PBR material + sun light + exposure + capture |
| S2 | Material visual truth | Terrain layers, material profile preview, texture residency, missing material fallback |
| S3 | Physical preview | Crater/spall/debris as authored preview and captured evidence |
| S4 | Weather visual | Rain/fog/sky cycle, storm visibility, backend fallback |
| S5 | Hydrology/fire sandbox | Barrel fill/leak/evaporation and wetness/fire visual channels |
| S6 | AI/living sandbox | Needs, squad intent, schedule, crime/status, causality trace seed |
| S7 | Integrated proof region | Small world area with graphics/material/physics/audio/diagnostics stitched |
| S8 | Large world expansion | Region/cell streaming, far representation, persistence, retained summaries |

## 4. Hard boundary decisions

### Engine owns
World truth, material truth, render truth, physics consequences, audio event truth, scheduler phases, save/restore state, runtime diagnostics.

### SDK owns
Typed packets, DTOs, handles, observations, command envelopes, failure codes, compatibility and normalization.

### Tooling owns
Import/cook/canonicalization, route execution, transaction ledgers, validation, capture, compare, benchmark, evidence, recovery.

### Editor owns
Operator surfaces, panels, viewport, inspector, content browser, labs, button focus, disabled reasons, user workflows.

### Apps own
Process launch, feature/profile selection, dependency wiring, window bootstrap, command invocation.

### Quality owns
Tests, proof packs, gates, audits, benchmark execution, regression evidence.

## 5. Mandatory anti-garbage law

The following are forbidden in active StratumX 1.0 work:

- backend-specific graphics types above backend boundary;
- source DCC files as runtime truth;
- `.blend` or `.max` parsed directly in runtime;
- editor mutating engine truth without SDK/tooling legality;
- app layer holding editor/product logic;
- tests outside `7.quality` except tiny local invariants;
- fake stub success;
- orphan active crates;
- placeholder packages with gold-ready labels;
- patch notes inside active `canonical/`;
- graph materials bypassing material profiles;
- plugin code bypassing import validation or command routes.

## 6. Decision update rule

Changing any decision in this document requires updates to:

1. `canonical/00_INDEX.md`
2. affected domain canonical manuals
3. developer_docs implementation manual
4. api_reference packet/command reference
5. tutorials and troubleshooting if user workflow changes
6. quality gate that proves the new rule
