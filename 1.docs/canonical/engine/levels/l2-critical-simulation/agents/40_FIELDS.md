# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| navigation_products | Bounded path/route records | Required | Regional/squad scope only, no full-world dense solve | `engine_agents` |
| perception_products | Bounded sensing records | Required | Tier-legal, near-first degradation, no raw sensor dump | `engine_agents` |
| action_intents | Bounded decision tokens | Required | Staged through world/apply, no direct mutation | `engine_agents` |
| society_deltas | Bounded social/economy records | Required | Slower cadence, no combat-rate global solve | `engine_agents` |
| schedule_products | Bounded routine/cadence records | Required | Bounded interrupt classes, no invalidation storms | `engine_agents` |

## Canonical Data Forms

All outputs are shaped bounded records only. Raw dumps, unbounded fanout, and hidden ownership transfers are illegal.

## Rules

Each field must:
- Respect simulation-tier law for near/far degradation
- Use bounded classes with explicit ceilings
- Stage mutations through world/apply law
- Preserve deterministic replay/persistence shape
- Degrade far/decorative work before near critical work
