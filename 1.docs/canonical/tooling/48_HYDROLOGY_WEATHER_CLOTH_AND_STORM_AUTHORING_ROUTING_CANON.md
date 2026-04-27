# Hydrology Weather Cloth And Storm Authoring Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the singular execution-grade tooling route for this heavy-domain family.
It owns legalization, routing, capture discipline, retry posture, rollback anchors, recovery mapping, focus hints, and retained artifact policy.

## Exact intent schema families
| Intent | Required fields | Output | First denial family |
|---|---|---|---|
| `intent.hydro.mass_ledger_compare` | scope, baseline id, compare horizon | mass-balance digest | `route.hydro.mass_compare_missing` |
| `intent.fire.weather_verify` | thermal cell group, wind bucket, smoke band | coupling digest | `route.fire.media_coupling_illegal` |
| `intent.soft_surface.response_review` | solver rung, wetness class, baseline id | response digest | `route.soft.response_compare_invalid` |

## Exact transaction state machine
`draft -> normalized -> baseline_bound -> slice_fetched -> legality_checked -> applied / reviewed -> captured -> compared -> recovery_selected -> certified / rolled_back / denied`

## Retry limits and rollback anchors
| Route slice | Retry limit | Rollback anchor |
|---|---:|---|
| hydro mass compare | 1 | restore anchor only |
| fire/weather verify | 1 | previous volumetric rung |
| soft response review | 1 | previous solver rung |

## Invalidation triggers
- container topology change
- storm band remap
- wind bucket remap
- wetness coefficient change
- replay horizon shortening

## Cache ownership
- hydro field cache
- media band cache
- soft-response cache
- compare cache
- artifact ledger

## Artifact ownership
- mass ledger snapshot
- coupling digest
- response rung snapshot
- triplet compare bundle

## Diagnostics envelope
Every terminal or retryable result must expose: first failure code, route state, rollback anchor id, next legal recovery action.

## Next legal recovery mapping
- success focus must target certification or evidence surfaces only when retained artifacts exist;
- retryable failure focus must target the owning lab plus one recovery action;
- terminal failure focus must target diagnostics or evidence review with one blocker trace and one rollback anchor.

## Current posture
`document_gold / doc_closed_impl_open`
