# Compiled Gate Program Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Canonical rule
Compatibility verdicts, legality decisions, and transport admissibility must resolve from immutable facts through declared compiled lookup tables.

## Inputs
- version facts
- capability facts
- profile facts
- declared transport policy records

## Outputs
- compatibility verdicts
- legality verdicts
- admissible publication shape

## Hot-path law
The hot write path may not rebuild legality graphs, profile graphs, or compatibility graphs per submission.
It may only evaluate declared compiled tables over immutable input tuples.

## Forbidden
- ad-hoc mutable verdict caches with hidden invalidation
- editor-owned fallback legality logic inside L5
- per-submission reconstruction of compatibility topology
