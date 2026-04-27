# World Identity And Namespace Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze world identity, namespace, and lineage law so that authoring, save/restore, build/export, and runtime validation all talk about the same world.

## Identity law
Every world package must declare and preserve:
- immutable `world_id`;
- human-readable `world_label`;
- `world_role` describing how the world participates in product startup or content flow;
- namespace roots for terrain, materials, environment, placements, diagnostics, and save-state families;
- lineage metadata for creation, import, migration, and last lawful mutation.

## Namespace law
No world package may rely on implicit editor-local paths or ad-hoc file placement.
All subordinate refs must be resolvable from the world manifest and must remain stable across save/restore, compare/capture, and build/export.

## Deny conditions
- missing or duplicate `world_id`;
- namespace collision between world families;
- orphaned subordinate refs;
- lineage gap after migration or import.
