# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| entity_id | Entity identity token | Required | Stable with generation-protected reuse | engine_identity |
| component_id | Component identity token | Required | Stable typed component identifier | engine_identity |
| generation | Generation counter | Required | Sufficient width for stale-handle aliasing protection | engine_identity |
| identity_domain | Domain classification | Required | Typed separation of entity, component, and adjacent spaces | engine_identity |

## Invariant Rules

- Entity identity is never a dense traversal token
- Entity identity never carries execution-lane locality assumptions
- Identity reuse is generation-protected and reuse-delayed
- Generation width must be sufficient to make stale-handle aliasing non-trivial under canonical stress tests
- Free-list reuse may not immediately recycle freshly retired identities into the same execution epoch
- Identity domains provide typed separation of identity namespaces
- Identity must stay separate from storage and access concerns
