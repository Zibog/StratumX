# Global Stack Map

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Layer Range Table

| Band | Layer range | Package | Short description |
|---|---|---|---|
| Foundation | L-0.05 – L-0.9, L-1 | engine | World region, ECS assembly/query/registry, storage, handle, identity |
| World truth | L0, L0.5 | engine | World truth and shared world properties |
| Runtime kernel | L1, L1.5 | engine | Runtime kernel + runtime resource services |
| Critical simulation | L2, L2.5 | engine | Critical simulation + network runtime services |
| Model and synthesis | L3.0 – L3.2 | engine | Model systems, synthesis systems, resource systems |
| Startup | L4 | engine | Startup and wiring |
| SDK bridge | L5.0 – L5.15 | sdk | Data-oriented public bridge: link packets/controls, observations/metrics, compat, transport, legality gates, handles, refs |
| Authority core | L6.0, L6.2 – L6.9 | tooling | Authority, transaction ledger, snapshot/index/derived/artifact/stream/cache/budget planes |
| Tool intent | L6.0t – L6.17t | tooling | Tool sessions, selections, focus refs, inspection views, preview/diagnostics, content/scene intents |
| Runtime group | L6.10 – L6.14 | tooling | Workspace, validation, preview, build, release runtimes |
| Assistant tooling | L6A.0 – L6A.7 | tooling | Assistant sessions, context packs, proposal/lowering/safety/apply-revert runtimes, model requests |
| Studio orchestration | L7.0 – L7.7 | tooling | Project orchestration, content/world/sim/release campaigns, automation/governance meta, reporting |
| Assistant brain | L7A.0 – L7A.6 | tooling | Prompt understanding, planning, canon reasoning, generation planning, optimization, migration, model routing |
| Editor shell | L8.0 – L8.11 | editor | Shell, viewport, panels, tool context, overlay/gizmo, layout, interaction routing, assistant surface, diagnostics, build/release surface |
| Authoring suites | L9.0 – L9.11 | editor | World, scene, terrain, material, destruction, simulation/AI, weather, animation, audio, UI/HUD, quest, build/validation/release suites |
| Editor services | L10.0 – L10.7 | editor | Project bootstrap, import/export, graph authoring, automation, script/hot-reload, plugin/extension host, template/scaffold, package/market |
| Collaboration | L11.0 – L11.5 | editor | Collaboration session, review/annotation, asset gate/approval, playtest/capture, production dashboard, learning/onboarding |

## Data-Flow Direction

```
engine (L-0 — L4)
    ↓  [ L5 SDK bridge — data-oriented read adapters only ]
sdk (L5)
    ↓  [ L6 planes read from L5 snapshots/refs ]
tooling (L6 – L7, L6A – L7A)
    ↓  [ L8 reads from tooling planes via legal refs/requests ]
editor (L8 – L11)
```

## Cross-Cut Rules

- No upward dependency is legal (editor may not directly access engine types).
- The SDK L5 layer is the **only** legal read surface for tooling to consume engine-truth.
- Tooling L6 planes are the **only** legal read surface for editor to consume SDK/engine-truth.
- Direct cross-cuts (editor → sdk, editor → engine, tooling → engine) are **forbidden**.
- See `09_GLOBAL_BOUNDARY_PRESERVATION_MATRIX.md` for exact allowed types at each boundary.
