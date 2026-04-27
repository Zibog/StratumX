# Item Equipment And Economy Authoring Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the singular execution-grade route for item state, equipment legality, trader/economy consequence, and checkpoint-linked routing.
It does not own engine truth and it does not redefine sdk packet families.

## Exact intent schema families
| Intent | Required fields | Output | First denial family |
|---|---|---|---|
| `intent.item.state_compare` | item identity set, baseline id, compare class | item-state compare digest | `route.item.identity_drift` |
| `intent.equipment.legality_review` | equipment loadout id, scope, legality tier | equipment legality verdict | `route.equipment.scope_invalid` |
| `intent.economy.consequence_review` | economy delta id, checkpoint id, baseline id | economy consequence bundle | `route.economy.delta_gap` |

## Transaction state machine
`draft -> normalized -> legality_checked -> applied -> captured -> compared -> reviewed -> certified / rolled_back`

## Retry and rollback posture
- retry limit: 1 retry for artifact read failure only;
- rollback target: restore item baseline, equipment baseline, or economy checkpoint delta only;
- replay or compare window: `checkpoint chain`.

## Invalidation triggers
- identity set change;
- equipment legality rule replacement;
- checkpoint delta replacement;
- economy normalization shift;

## Cache ownership
| Cache | Authoritative content |
|---|---|
| item cache | item state and compare digests |
| equipment cache | loadout legality bundles |
| economy cache | checkpoint-linked delta bundles |
| artifact ledger | retained compare bundles |

## Artifact ownership
Retained artifacts for this route are:
- item identity digest, equipment legality bundle, economy consequence bundle.
Failed runs may never overwrite the last-good baseline.

## Diagnostics envelope
Every execution of this route must publish:
- route id, item/equipment/economy scope, first blocking code, baseline ids, next legal recovery action.

## Recovery action mapping
| Failure code | Next legal recovery action |
|---|---|
| `route.item.identity_drift` | restore item baseline and rerun compare |
| `route.equipment.scope_invalid` | normalize equipment scope and rerun legality review |
| `route.economy.delta_gap` | restore prior checkpoint delta and rerun consequence review |
