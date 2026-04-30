# Runtime Data Carriers and Truth Boundary Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Name the data carriers that cross stack boundaries.

## Core statements
- truth objects remain engine-owned
- observations and results are sdk-owned data carriers
- transactions and retries are tooling-owned
- panel state and local toggles are editor-owned

## Law
No package may smuggle ownership through unnamed blobs.

## Required companion docs
- `13_STACK_CAPABILITY_ATLAS.md`
- `14_TASK_ROUTING_AND_OWNERSHIP_CANON.md`
- `15_IMPLEMENTATION_REALITY_AND_STATUS_LEDGER.md`
- `16_STACK_WORKFLOW_CLOSURE_MATRIX.md`

---

# V34 world/entity/scheduler implementation decisions

Stack version: `SX-CANON/1.0.28/STACK-v34`

## World hierarchy

`World → Region → Sector → Cell → Chunk → Entity/Surface/Field`.

Authoring coordinates may use stable high precision global positions. Runtime hot simulation uses cell-local coordinates. Streaming loads chunks; regions are packaging/authoring scopes.

## Entity model

`WorldEntity` is stable identity. Components are data. Systems own behavior. Editor object is authoring projection, not runtime truth.

## Scheduler model

Engine owns the phase scheduler. Domains publish jobs into phases. Domains do not spawn arbitrary global threads.

Required phases:

1. Input
2. AuthoringCommands
3. WorldMutation
4. Simulation
5. Physics
6. AI
7. Animation
8. AudioPrepare
9. RenderExtract
10. RenderSubmit
11. Diagnostics
12. Persistence

## Physics strategy

StratumX may use a replaceable physics kernel adapter for collision/motion. StratumX truth remains above it in material response, topology, field substrate, aftermath, diagnostics, and persistence.
