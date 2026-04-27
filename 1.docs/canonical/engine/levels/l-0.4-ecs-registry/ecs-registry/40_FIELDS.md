# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| entity_set | Entity identity set | Required | Structural existence truth | engine_ecs_registry |
| component_classes | Component type set | Required | Structural component class registry | engine_ecs_registry |
| presence_map | Entity-Component presence | Required | Structural attachment truth | engine_ecs_registry |
| membership_map | Entity-ComponentSet mapping | Required | Structural membership binding | engine_ecs_registry |
| registry_tables | Registration state | Required | Canonical structural truth | engine_ecs_registry |

## Invariant Rules

- Registry owns structural shape only
- Registry must not own steady-state traversal logic
- Registry must not own convenience lookups
- Registry must not own compiled lane execution
- Presence truth must be explicit and stable
- Membership maps must reflect structural attachment only
