# World Scale Performance Budget And Degradation Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the singular execution-grade tooling route for this heavy-domain family.
It owns legalization, routing, capture discipline, retry posture, rollback anchors, recovery mapping, focus hints, and retained artifact policy.

## Exact intent schema families
| Intent | Required fields | Output | First denial family |
|---|---|---|---|
| `intent.floor.pack_run` | pack id, axis set, baseline pointer | floor pack result | `route.floor.pack_missing` |
| `intent.floor.recovery_step` | axis, ladder step, baseline pointer | recovery legality | `route.floor.axis_red_unresolved` |

## Exact transaction state machine
`draft -> normalized -> baseline_bound -> slice_fetched -> legality_checked -> applied / reviewed -> captured -> compared -> recovery_selected -> certified / rolled_back / denied`

## Retry limits and rollback anchors
| Route slice | Retry limit | Rollback anchor |
|---|---:|---|
| floor pack run | 1 | last-good baseline only |
| floor recovery step | 0 | previous ladder step |

## Invalidation triggers
- threshold row replacement
- ladder reorder
- axis bucket reclassification
- baseline pointer loss

## Cache ownership
- pressure timeline cache
- degrade ladder cache
- certification cache
- artifact ledger

## Artifact ownership
- pressure timeline
- failed-run bundle
- recovery-run bundle
- last-good baseline bundle

## Diagnostics envelope
Every terminal or retryable result must expose: axis, route state, threshold row id, first failure code, next legal recovery action.

## Next legal recovery mapping
- success focus must target certification or evidence surfaces only when retained artifacts exist;
- retryable failure focus must target the owning lab plus one recovery action;
- terminal failure focus must target diagnostics or evidence review with one blocker trace and one rollback anchor.

## Current posture
`document_gold / doc_closed_impl_open`
