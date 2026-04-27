# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| contact_products | Bounded contact pair records | Required | Regional/batch scope only, no unbounded same-tick coupling | `engine_kinetics` |
| motion_deltas | Bounded rigidbody motion records | Required | Island-scoped, staged through world/apply, no direct mutation | `engine_kinetics` |
| trajectory_products | Bounded projectile solve records | Required | Explicit fast-lane classes, no always-max fidelity for all | `engine_kinetics` |
| impact_products | Bounded impact aftermath records | Required | Bounded same-tick records, no foreign-family ownership | `engine_kinetics` |

## Canonical Data Forms

All outputs are shaped bounded records only. Raw dumps, unbounded coupling, and hidden ownership transfers are illegal.

## Rules

Each field must:
- Respect simulation-tier law for near/far degradation
- Use bounded classes with explicit ceilings (islands, contacts, projectiles)
- Stage mutations through world/apply law
- Preserve deterministic replay/persistence shape
- Degrade far/decorative work before near critical work
- Obey cross-family bridge law for same-tick coupling
