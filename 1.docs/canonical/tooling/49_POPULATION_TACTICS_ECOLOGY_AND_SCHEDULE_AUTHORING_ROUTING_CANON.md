# Population Tactics Ecology And Schedule Authoring Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the singular execution-grade tooling route for this heavy-domain family.
It owns legalization, routing, capture discipline, retry posture, rollback anchors, recovery mapping, focus hints, and retained artifact policy.

## Exact intent schema families
| Intent | Required fields | Output | First denial family |
|---|---|---|---|
| `intent.society.schedule_audit` | scope, time band, baseline id | schedule legality digest | `route.society.scope_invalid` |
| `intent.tactics.cover_review` | cover graph id, rollback anchor, compare class | tactics digest | `route.tactics.cover_graph_invalid` |
| `intent.ecology.route_audit` | corridor graph id, hazard tier, baseline id | migration digest | `route.ecology.route_missing` |

## Exact transaction state machine
`draft -> normalized -> baseline_bound -> slice_fetched -> legality_checked -> applied / reviewed -> captured -> compared -> recovery_selected -> certified / rolled_back / denied`

## Retry limits and rollback anchors
| Route slice | Retry limit | Rollback anchor |
|---|---:|---|
| society audit | 1 | previous legality anchor |
| tactics review | 0 | previous cover graph only |
| ecology route audit | 1 | previous route tier |

## Invalidation triggers
- schedule slot rebinding
- crime bucket rebucket
- cover graph replacement
- hazard tier remap
- corridor edge replacement

## Cache ownership
- society ledger cache
- tactics graph cache
- ecology corridor cache
- compare cache
- artifact ledger

## Artifact ownership
- schedule legality board
- cover graph snapshot
- migration route snapshot
- triplet compare bundle

## Diagnostics envelope
Every terminal or retryable result must expose: scope id, route state, first failure code, next action id, focus target id.

## Next legal recovery mapping
- success focus must target certification or evidence surfaces only when retained artifacts exist;
- retryable failure focus must target the owning lab plus one recovery action;
- terminal failure focus must target diagnostics or evidence review with one blocker trace and one rollback anchor.

## Current posture
`document_gold / doc_closed_impl_open`
