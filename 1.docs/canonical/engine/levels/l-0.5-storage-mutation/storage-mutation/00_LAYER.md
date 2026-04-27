# engine_storage_mutation

## Stack Position

L-0.5. Storage Mutation

## Primary Role

Staging and packaging of storage-side mutations.

## Canonical Scope

Mutation buffers, deferred writes, change sets, and apply payloads.

## Boundary Rationale

Mutation staging is a separate pressure axis because hot writes must be packaged and ordered independently of layout and access.

## Canonical Consumers

- `engine_ecs`
- `engine_world`
- `engine_runtime`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_handle` — Stable references.
- `engine_storage_layout` — Layout structure.
- `engine_storage_access` — Legal access windows.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Mutation Buffers | Temporary batched mutation structures. | `40_MUTATION_BUFFERS.md` |
| Deferred Writes | Deferred storage write model. | `41_DEFERRED_WRITES.md` |
| Change Sets | Packaged structural and data changes. | `42_CHANGE_SETS.md` |
| Apply Payloads | Payloads prepared for controlled apply. | `43_APPLY_PAYLOADS.md` |
