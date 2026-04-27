# Ref Subtypes

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Handle subtypes
- session handle
- object handle
- runtime handle

## Ref subtypes
- identity ref
- state ref
- artifact ref

## Law
Handles point at live scoped engine-side lifetimes.
Refs point at read-side projections.
Artifact refs point at deterministic generated products.
None of them carry mutation authority by themselves.
