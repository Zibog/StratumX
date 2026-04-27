# Boundary Preservation

## Canonical Rule

`engine_storage_mutation` provides staging and packaging of storage-side mutations. Hot writes must be packaged and ordered independently of layout and access to maintain separation of concerns.

## Upward Boundary

**Exports to layers above:**
- Mutation buffers (batched mutation staging)
- Deferred writes (write deferral structures)
- Change sets (structured mutation packages)
- Apply payloads (ordered, tagged batches for segmented apply)

**Canonical consumers:**
- `engine_ecs` — ECS assembly layer
- `engine_world` — World truth layer
- `engine_runtime` — Runtime kernel

## Downward Boundary

**Imports from layers below:**
- `engine_core` — Base contracts
- `engine_handle` — Stable references
- `engine_storage_layout` — Layout structure
- `engine_storage_access` — Legal access windows

## Forbidden Crossings

**Explicitly forbidden boundary violations:**
- Direct world mutation during compute phases (must use deferred writes)
- Random micro-writes bypassing batch accumulation
- Whole-world monolithic integration when regional/island-scoped integration is legal
- Bypassing merge/coalesce rules for mutation buffers
- Ignoring idempotence classes in batch ordering
- Direct access to identity layer (must go through handle)
