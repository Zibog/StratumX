# engine_storage_layout

## Stack Position

L-0.7. Storage Layout

## Primary Role

Physical organization of storage.

## Canonical Scope

Chunk layout, column layout, locality model, and memory organization policy.

## Boundary Rationale

Layout is a standalone pressure axis because memory shape must stay independent from access model, mutation staging, and query logic.

## Canonical Consumers

- `engine_storage_access`
- `engine_storage_mutation`
- `engine_ecs_registry`
- `engine_ecs`
- `engine_runtime`

## Downward Dependencies

- `engine_core` — Base contracts and types.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Layout Model | Canonical physical storage model. | `40_LAYOUT_MODEL.md` |
| Chunk Layout | Chunk-based storage organization. | `41_CHUNK_LAYOUT.md` |
| Column Layout | Column-oriented storage organization. | `42_COLUMN_LAYOUT.md` |
| Locality Model | Cache and locality-oriented storage structure. | `43_LOCALITY_MODEL.md` |
