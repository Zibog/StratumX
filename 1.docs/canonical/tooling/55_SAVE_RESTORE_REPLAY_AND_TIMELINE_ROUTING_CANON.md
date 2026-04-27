# Save Restore Replay And Timeline Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the singular execution-grade tooling route for this heavy-domain family.
It owns legalization, routing, capture discipline, retry posture, rollback anchors, recovery mapping, focus hints, and retained artifact policy.

## Exact intent schema families
| Intent | Required fields | Output | First denial family |
|---|---|---|---|
| `intent.timeline.restore_triplet` | anchor id, replay window, compare class | restore digest | `route.timeline.anchor_missing` |
| `intent.timeline.capture_manifest` | anchor id, artifact class | capture manifest | `route.timeline.artifact_missing` |

## Exact transaction state machine
`draft -> normalized -> baseline_bound -> slice_fetched -> legality_checked -> applied / reviewed -> captured -> compared -> recovery_selected -> certified / rolled_back / denied`

## Retry limits and rollback anchors
| Route slice | Retry limit | Rollback anchor |
|---|---:|---|
| restore triplet | 1 | previous restore anchor |
| capture manifest | 1 | last-good artifact ref |

## Invalidation triggers
- anchor replacement
- replay window shrink below certification floor
- artifact manifest mutation

## Cache ownership
- timeline anchor cache
- replay triplet cache
- artifact manifest cache

## Artifact ownership
- restore anchor manifest
- triplet bundle
- artifact capture bundle

## Diagnostics envelope
Every terminal or retryable result must expose: anchor id, route state, first failure code, artifact ref, next legal recovery action.

## Next legal recovery mapping
- success focus must target certification or evidence surfaces only when retained artifacts exist;
- retryable failure focus must target the owning lab plus one recovery action;
- terminal failure focus must target diagnostics or evidence review with one blocker trace and one rollback anchor.

## Current posture
`document_gold / doc_closed_impl_open`
