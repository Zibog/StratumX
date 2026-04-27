# Detached Fragment Lifecycle And Debris Bed Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own fragment truth after destruction events so fragments stop being half-VFX, half-runtime magic.

## Fragment classes
- `structural_chunk`
- `surface_panel`
- `slab_piece`
- `splinter`
- `shard`
- `dust_mass`

## Lifecycle
`released -> promoted_physics / simplified_fall -> settling -> sleeping -> merged_to_host / persisted_as_debris_bed / culled_with_digest`

## Exact law
- release class determines spawn budget and physics richness;
- settle detection must publish the rule that ended dynamic motion;
- merge-to-host is legal only when traversal/collision/visual consequences remain preserved;
- debris-bed accumulation is a first-class truth conversion, not a discarded visual leftover.

## Failure families
- `fragment.spawn_budget_fraud`
- `fragment.settle_rule_missing`
- `fragment.merge_legality_break`
- `fragment.debris_bed_gap`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
