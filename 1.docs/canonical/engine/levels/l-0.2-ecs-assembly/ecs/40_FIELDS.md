# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| entity_descriptor | EntityDescriptor | required | composed entity truth from component bundles | owned by `engine_ecs` |
| component_bundle | ComponentBundle | required | grouped component state for entity | owned by `engine_ecs` |
| composition_template | CompositionTemplate | optional | reusable assembly pattern | owned by `engine_ecs` |
| entity_archetype | ArchetypeIdentifier | required | archetype signature for storage dispatch | owned by `engine_ecs` |
