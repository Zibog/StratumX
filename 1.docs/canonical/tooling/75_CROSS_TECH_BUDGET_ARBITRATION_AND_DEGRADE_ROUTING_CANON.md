# Cross-Tech Budget Arbitration and Degrade Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the tooling execution contract for the `arbitration` family.

## Required intent schema
- `resource_vectors` — vector set
- `owner_phase_set` — phase set
- `requested_mode` — observe/degrade/recover/cert
- `fallback_profile` — profile target
- `expected_result_kind` — arbitration verdict

## Transaction state machine
| State | Meaning |
|---|---|
| received | intent accepted into tooling |
| validated | preconditions checked |
| normalized | schema/profile normalization completed |
| prepared | route plan, cache plan, artifact plan resolved |
| executing | route is active |
| publishing | results, focus, and diagnostics are being emitted |
| retryable_failure | bounded retry is legal |
| rolled_back | rollback completed |
| completed | terminal success |
| terminal_failure | terminal failure with retained artifacts |

## Retry and rollback law
- retries are bounded and must retain the original `request_id` lineage;
- rollback policy must declare cache invalidation and artifact retention;
- terminal failure must still emit diagnostics and next legal action.

## Cache and artifact ownership
Tooling owns normalization caches, compare caches, and artifact reveal/retention policy for this family.
Runtime truth remains engine-owned.

## Current posture
`document_gold / doc_closed_impl_open`
