# Wound Layer Species And Biological Consequence Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own species-aware layered wound authoring and debug.

## Mandatory controls
- inspect body topology class;
- preview layered traversal;
- toggle gore legality families;
- inspect survivability and limb-severance outcomes;
- replay wound chain;
- inspect healing or scarring persistence.

## Required inspector fields
- `species_topology_class`
- `layer_entry_ref`
- `organ_zone_ref`
- `bone_hit_flag`
- `gore_legality_code`
- `survivability_verdict`
- `healing_or_scarring_ref`

## Disabled reasons
`WND_DISABLED_NO_BODY_SCOPE`, `WND_DISABLED_SPECIES_TOPOLOGY_UNKNOWN`, `WND_DISABLED_TRAVERSAL_CHAIN_MISSING`
