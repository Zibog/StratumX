# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| material_descriptor | Material identity descriptor | Required | Stable identity, classes, property attachment | engine_material |
| material_id | Compact material identifier | Required | Compact immutable id | engine_material |
| property_domain | Property classification | Required | Physical, thermal, fluid, structural, acoustic, appearance | engine_material |
| response_profile | Reaction row mapping | Required | Compact id to reaction rows | engine_material |
| descriptor_row | Descriptor lookup row | Required | Compact immutable id, bounded hot cache | engine_material |
| reaction_row | Reaction lookup row | Required | Flattened immutable row, bounded hot cache | engine_material |
| lookup_cache | Material lookup cache | Optional | Diagnostics-law bounded only | engine_material |
| fallback_descriptor | Descriptor fallback | Required | One deterministic fallback step | engine_material |
| default_reaction | Reaction fallback | Required | Deterministic authored default | engine_material |

## Invariant Rules

- Response profiles are immutable at runtime unless changed through authoritative apply or legal content reload paths
- Response-profile lookup must remain O(1)-like and cache-friendly for hot paths
- Material consumers may not duplicate ownership of response law
- Descriptor chasing through long ownership chains is illegal in critical execution lanes
- Descriptor miss allows one deterministic fallback step only (no recursive owner-chain walk)
- Reaction miss uses deterministic authored default (no dynamic query chase)
- Cache miss ledger is optional diagnostics-only (no persistent hot-path ledger)
- Lookup is read-heavy, compact-id based, flattened, and cache-friendly
- Material is read-heavy shared substrate, not a regular world-progression family
