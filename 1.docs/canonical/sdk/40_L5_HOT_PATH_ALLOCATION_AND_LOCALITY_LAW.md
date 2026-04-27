# L5 Hot-Path Allocation And Locality Law

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Canonical rule
L5 hot write and hot read publication paths must preserve locality, bounded allocation posture, and explicit hot/cold separation.

## Required posture
- headers stay compact and typed
- routing keys stay separate from cold diagnostics
- payload refs stay cheaper than payload-body ownership where possible
- snapshot reads stay immutable
- batch fanout stays immutable

## Forbidden posture
- hidden heap growth on every packet or observation envelope by default
- pointer-rich editor-shaped structs inside bridge hot paths
- mixing compatibility facts, mutable queues, and ref registries inside one mega-structure
- silent duplication of the same engine truth across multiple mutable arrays

## Audit rule
Any intentional hot-path allocation, copying point, or widening point must be explicit, justified, and pressure-visible.
