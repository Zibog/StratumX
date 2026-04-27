# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| field_deltas | Bounded field change records | Required | Regional scope only, no planet-scale full solve every tick | `engine_field` |
| region_deltas | Bounded region-local world changes | Required | Staged through world/apply, no direct mutation | `engine_field` |
| structural_products | Bounded destruction records | Required | Bounded debris spill, no hidden secondary solver loops | `engine_field` |
| atmospheric_products | Bounded weather/wind records | Required | Regional scope, no synthesis ownership grab | `engine_field` |
| fluid_field_state | Bounded fluid distribution records | Required | Regional tiles only, no global fluid pass every tick | `engine_field` |
| thermal_field_state | Bounded thermal solve records | Required | Bounded cadence, no frame-rate full thermal solve | `engine_field` |
| terrain_field_state | Bounded terrain deformation records | Required | Regional/batch scope, no unbounded cross-chunk spill | `engine_field` |

## Canonical Data Forms

All outputs are shaped bounded records only. Raw dumps, unbounded spill, and hidden ownership transfers are illegal.

## Rules

Each field must:
- Respect simulation-tier law for near/far degradation
- Use bounded regional/batch scope with explicit ceilings
- Stage mutations through world/apply law
- Preserve deterministic replay/persistence shape
- Degrade far/decorative work before near critical work
- Obey cross-family bridge law for same-tick coupling
- Never steal synthesis ownership (imaging, acoustics, startup warm)
