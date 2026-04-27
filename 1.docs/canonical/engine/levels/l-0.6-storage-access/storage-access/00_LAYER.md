# engine_storage_access

## Stack Position

L-0.6. Storage Access

## Primary Role

Deterministic read/write access model over storage layout.

## Canonical Scope

Read views, write windows, access modes, and traversal entry surfaces.

## Boundary Rationale

Access must stay independent from layout and mutation staging so the engine can keep read/write legality explicit and schedulable.

## Canonical Consumers

- `engine_storage_mutation`
- `engine_ecs_query`
- `engine_ecs`
- `engine_runtime`
- `engine_agents`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_handle` — Stable references.
- `engine_storage_layout` — Physical storage structure.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Read Views | Read-only access surfaces over storage. | `40_READ_VIEWS.md` |
| Write Windows | Controlled mutable access windows. | `41_WRITE_WINDOWS.md` |
| Access Modes | Access legality and mode descriptors. | `42_ACCESS_MODES.md` |
| Traversal Entry | Deterministic traversal entry surfaces. | `43_TRAVERSAL_ENTRY.md` |
