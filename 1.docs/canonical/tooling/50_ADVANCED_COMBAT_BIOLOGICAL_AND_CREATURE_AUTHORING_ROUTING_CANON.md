# Advanced Combat Biological And Creature Authoring Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the singular execution-grade tooling route for this heavy-domain family.
It owns legalization, routing, capture discipline, retry posture, rollback anchors, recovery mapping, focus hints, and retained artifact policy.

## Exact intent schema families
| Intent | Required fields | Output | First denial family |
|---|---|---|---|
| `intent.wound.trace_review` | trace id, species profile, baseline id | wound replay digest | `route.wound.trace_missing` |
| `intent.creature.damage_profile_review` | creature scope, lethality class, compare class | creature damage digest | `route.creature.profile_missing` |

## Exact transaction state machine
`draft -> normalized -> baseline_bound -> slice_fetched -> legality_checked -> applied / reviewed -> captured -> compared -> recovery_selected -> certified / rolled_back / denied`

## Retry limits and rollback anchors
| Route slice | Retry limit | Rollback anchor |
|---|---:|---|
| wound trace review | 1 | wound recovery anchor |
| creature profile review | 0 | previous legality slice |

## Invalidation triggers
- trace-link replacement
- species profile change
- dismemberment ruleset rebinding

## Cache ownership
- wound replay cache
- creature damage cache
- compare cache
- artifact ledger

## Artifact ownership
- wound replay triplet
- species profile snapshot
- region-trace capture

## Diagnostics envelope
Every terminal or retryable result must expose: trace id, route state, first failure code, rollback anchor, next legal recovery action.

## Next legal recovery mapping
- success focus must target certification or evidence surfaces only when retained artifacts exist;
- retryable failure focus must target the owning lab plus one recovery action;
- terminal failure focus must target diagnostics or evidence review with one blocker trace and one rollback anchor.

## Current posture
`document_gold / doc_closed_impl_open`
