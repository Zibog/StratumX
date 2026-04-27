# Generative Dialogue And Runtime Policy Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the singular execution-grade tooling route for this heavy-domain family.
It owns legalization, routing, capture discipline, retry posture, rollback anchors, recovery mapping, focus hints, and retained artifact policy.

## Exact intent schema families
| Intent | Required fields | Output | First denial family |
|---|---|---|---|
| `intent.semantic.chain_audit` | intent id, policy tier, baseline anchor | semantic digest | `route.semantic.guard_fail` |
| `intent.semantic.recovery_preview` | consequence chain id, rollback anchor | recovery verdict | `route.semantic.baseline_missing` |

## Exact transaction state machine
`draft -> normalized -> baseline_bound -> slice_fetched -> legality_checked -> applied / reviewed -> captured -> compared -> recovery_selected -> certified / rolled_back / denied`

## Retry limits and rollback anchors
| Route slice | Retry limit | Rollback anchor |
|---|---:|---|
| semantic chain audit | 1 | semantic baseline anchor |
| semantic recovery preview | 0 | previous consequence chain |

## Invalidation triggers
- policy gate replacement
- memory delta rewrite
- consequence chain relink

## Cache ownership
- semantic guard cache
- consequence chain cache
- compare cache
- artifact ledger

## Artifact ownership
- guard board
- semantic triplet bundle
- consequence chain manifest

## Diagnostics envelope
Every terminal or retryable result must expose: guard id, route state, first failure code, next action id, focus target id.

## Next legal recovery mapping
- success focus must target certification or evidence surfaces only when retained artifacts exist;
- retryable failure focus must target the owning lab plus one recovery action;
- terminal failure focus must target diagnostics or evidence review with one blocker trace and one rollback anchor.

## Current posture
`document_gold / doc_closed_impl_open`
