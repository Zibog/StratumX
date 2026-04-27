# Creature Ecology Territory And Multi Species Persistence Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own food-chain, territory, den, migration, breeding, attrition, disease, and herd/pack identity law across long horizons.

## Exact truth objects

| Object | Role |
|---|---|
| `EcologyPopulationLedger` | count, age, vitality, and stress by species group |
| `TerritoryClaimSet` | claimed region, overlap, and conflict posture |
| `DenSiteLedger` | den, nest, or lair location with continuity state |
| `MigrationTriggerSet` | hunger, weather, hazard, breeding, or pressure triggers |
| `PredatorPreyEventLedger` | consumption, conflict, and chase consequence chain |
| `AgingAttritionLedger` | age pressure, injury weakness, and non-combat loss |
| `PackHerdIdentitySet` | persistent group identity across streaming and save/restore |

## Update order
`population tick -> territory refresh -> trigger evaluation -> migration or hunt decision -> consequence emission -> breeding/attrition update -> persistence publication`

## Publications
- `packet.living.ecology_state.v1`
- `packet.living.migration_front.v1`
- `packet.living.predator_prey_event.v1`

## Failure families
- `ecology.food_chain_gap`
- `ecology.territory_conflict_unresolved`
- `ecology.migration_trigger_missing`
- `ecology.pack_identity_drift`
- `ecology.persistence_restore_gap`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
