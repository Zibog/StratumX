# Material Persistence And Aftermath Response Registry Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the registry-grade law for `matresp.persistence.*` so persistence is no longer a thin companion note next to stronger visual/audio/light branches.

## Master law
Material persistence is owned by material law and expressed through world law.
It decides what survives save/restore, what may be rebuilt on load, and what aftermath meaning must remain canonical even when rich local simulation is gone.

## Mandatory persistence family buckets
| Family id bucket | Meaning |
|---|---|
| `matresp.persistence.none_retainable.*` | branch is preview-only and must not persist |
| `matresp.persistence.overlay_retained.*` | overlay/aftermath refs persist, rich media stays derived |
| `matresp.persistence.damage_retained.*` | structural/material damage state persists |
| `matresp.persistence.wetness_retained.*` | wetness/saturation state persists according to world policy |
| `matresp.persistence.thermal_retained.*` | char/scorch/heat aftermath persists where declared |
| `matresp.persistence.full_state_retained.*` | exact branch state is part of world-facing durable truth |

## Required fields
Every persistence family row must publish:
- `family_id`
- `retained_truth_classes`
- `save_scope`
- `restore_scope`
- `derivable_on_load_classes`
- `aftermath_overlay_policy`
- `world_storage_fields`
- `invalid_combination_refs`
- `cheap_runtime_interaction`
- `validation_gate_family`

## World storage law
World packages may retain only:
- canonical family refs
- lawful state modifiers
- retained aftermath refs
- declared cheap-runtime posture refs when persistence requires them
- save/restore metadata that world law names as durable

World packages may not retain:
- renderer-only transient noise
- audio substitute ids
- preview-only local solve traces
- temporary capture artifacts

## Validation law
Persistence fails validation when:
- a branch declares retained truth but no world storage fields;
- cheap-runtime rung would discard a state the persistence family marks mandatory;
- aftermath overlay is persisted without a lawful family or world retention policy.
