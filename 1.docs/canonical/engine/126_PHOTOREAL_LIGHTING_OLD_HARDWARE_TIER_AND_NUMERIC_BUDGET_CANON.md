# Photoreal Lighting Old Hardware Tier And Numeric Budget Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Turn “photoreal on old hardware” into a numeric contract.

## Feature ladders
| Family | Tier law |
|---|---|
| local lights | full shadowed -> mixed shadowed -> selected shadowed -> baked/probe-only fallback |
| muzzle-flash lighting | transient dynamic -> clustered transient -> intensity-only probe kick -> emissive-only fallback |
| vegetation shadowing | per-instance -> grouped -> impostor-caster -> ambient-only fallback |
| volumetrics | full ray-marched -> quarter-res temporal -> slice-based -> fog-only fallback |
| reflection | probe + screen -> probe-only -> atlas-reduced -> specular family fallback |
| probe updates | continuous -> throttled -> importance-queued -> frozen with legality badge |

## Numeric scene budgets
| Scene class | Band | Green | Red |
|---|---|---:|---:|
| old-PC tunnel firefight | lighting/media/post total GPU | `<= 8.5 ms` | `> 10.0 ms` |
| old-PC tunnel firefight | dynamic shadows | `<= 2.2 ms` | `> 3.4 ms` |
| old-PC tunnel firefight | volumetrics | `<= 1.4 ms` | `> 2.2 ms` |
| old-PC tunnel firefight | transient emission | `<= 0.35 ms` | `> 0.7 ms` |
| wet forest dusk | foliage+shadow band | `<= 2.8 ms` | `> 4.0 ms` |

## Publication law
Every downgrade must publish:
- scene class id;
- hardware profile id;
- current tier for each affected family;
- first blocked feature;
- first downgrade reason.

## Failure families
- `lighting.fallback_hidden`
- `lighting.old_floor_budget_red`
- `lighting.transient_emission_unbounded`
- `lighting.shadow_tier_desync`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
