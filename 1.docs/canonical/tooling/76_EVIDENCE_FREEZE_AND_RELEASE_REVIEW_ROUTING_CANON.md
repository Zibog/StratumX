# Evidence Freeze And Release Review Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file is the authoritative execution-grade route for evidence, freeze, and release review.
It is the only active truth for this route family.

## Route families
- `route.evidence.capture_review`
- `route.evidence.triplet_compare`
- `route.evidence.freeze_review`
- `route.evidence.signoff`

## Intent schema families
| intent family | Required fields |
|---|---|
| `intent.evidence.capture_review` | request id, route id, pack id, artifact ref, focus target id |
| `intent.evidence.triplet_compare` | request id, baseline ref, failed run ref, recovery run ref, compare mode |
| `intent.evidence.freeze_review` | request id, pack id, retained bundle ref, blocker trace, signoff scope |
| `intent.evidence.signoff` | request id, retained bundle ref, freeze signal, signoff actor |

## Transaction state machine
`state.received -> state.validated -> state.normalized -> state.bound -> state.executing -> state.partial_result? -> state.retryable_failure? -> state.rolled_back? -> state.recovered? -> state.completed / state.terminal_failure`

## Retry and rollback law
- retry budget family is `retry.compare_triplet` or `retry.cert_pack` only;
- rollback anchor must point to retained baseline or declared rollback artifact only;
- failed-run artifacts are never discarded before freeze review terminates;
- signoff may not silently normalize failure families.

## Cache and artifact ownership
- tooling owns review bundle assembly, triplet compare cache, release review bundle, and signoff emission policy;
- engine remains owner of runtime truth and raw diagnostics carriers;
- editor remains owner of visible focus and dashboard presentation only.

## Diagnostics and result envelopes
Every terminal state must publish:
- `artifact_ref` or retained bundle ref;
- `trace_ref` when blocker/failure is present;
- `focus_target_id`;
- `next_action_id`;
- `freeze_signal` when route class is review or signoff.

## Failure families
- `CRT_RETAINED_ARTIFACT_GAP`
- `CRT_TRIPLET_INCOMPLETE`
- `DISABLED_FREEZE_BLOCKER_OPEN`
- `DISABLED_BASELINE_MISSING`

## Law
No evidence/freeze/release review route may exist outside this file and `tooling/81`.
