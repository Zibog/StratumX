# Quest Event Authoring And World Consequence Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the singular execution-grade route for quest event chains, checkpoint access, and world consequence routing.
It does not own engine truth and it does not redefine sdk packet families.

## Exact intent schema families
| Intent | Required fields | Output | First denial family |
|---|---|---|---|
| `intent.quest.chain_compare` | quest chain id, baseline id, compare class | quest-chain compare digest | `route.quest.chain_gap` |
| `intent.event.checkpoint_access_review` | checkpoint id, access class, legality tier | checkpoint legality verdict | `route.quest.checkpoint_access_gap` |
| `intent.world.consequence_apply` | delta bundle id, restore class, evidence duty | consequence apply verdict | `route.quest.consequence_unretained` |

## Transaction state machine
`draft -> normalized -> legality_checked -> applied -> captured -> compared -> reviewed -> certified / rolled_back`

## Retry and rollback posture
- retry limit: 1 retry for missing compare mirror only;
- rollback target: restore quest-chain baseline or checkpoint delta baseline only;
- replay or compare window: `quest chain + restore`.

## Invalidation triggers
- quest graph change;
- checkpoint access rule change;
- world consequence delta mismatch;
- baseline pointer replacement;

## Cache ownership
| Cache | Authoritative content |
|---|---|
| quest cache | quest-chain and event digests |
| checkpoint cache | checkpoint legality bundles |
| consequence cache | world delta bundles |
| artifact ledger | retained compare bundles |

## Artifact ownership
Retained artifacts for this route are:
- quest-chain digest, checkpoint legality verdict, world consequence bundle.
Failed runs may never overwrite the last-good baseline.

## Diagnostics envelope
Every execution of this route must publish:
- route id, quest/event scope, active compare class, first blocking code, baseline ids, next legal recovery action.

## Recovery action mapping
| Failure code | Next legal recovery action |
|---|---|
| `route.quest.chain_gap` | capture quest baseline and rerun compare |
| `route.quest.checkpoint_access_gap` | restore prior checkpoint policy and rerun legality review |
| `route.quest.consequence_unretained` | deny freeze until retained consequence bundle exists |
