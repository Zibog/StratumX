# VFX Runtime Taxonomy Promotion And Truth Separation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own VFX as a first-class heavy domain rather than a bag of ad-hoc emitters.

## Canonical classes

| Class | Meaning | Default truth relation |
|---|---|---|
| debris | detached fragments, chips, and collapse dust | consequence-bound |
| sparks | short-lived emission from impacts or electrical events | presentational or consequence-bound |
| smoke | sustained volumetric byproduct | consequence-bound or promoted-local |
| fluid_spray | leak and impact spray | consequence-bound |
| ash | retained or semi-retained burn byproduct | retained-aftermath |
| dust | impact or traversal particulate | presentational or consequence-bound |
| transient_heat_glow | short-lived thermal light signature | presentational with diagnostics |

## Promotion law
`cheap_presentational -> consequence_bound -> promoted_local_sim -> retained_aftermath`

A class may move upward only when the owning runtime truth demands it.
VFX may reflect truth, enrich truth, or visualize truth; it may not silently become the only copy of truth.

## Budget law
Thousands of concurrent micro-events are legal only when promotion policy, merge policy, and downgrade publication are explicit.

## Failure families
- `vfx.class_taxonomy_missing`
- `vfx.promotion_law_break`
- `vfx.truth_separation_violation`
- `vfx.concurrent_microevent_budget_red`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
