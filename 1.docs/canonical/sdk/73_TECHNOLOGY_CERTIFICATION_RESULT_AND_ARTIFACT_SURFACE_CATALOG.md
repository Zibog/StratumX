# Technology Certification Result And Artifact Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the exact typed bridge for certification execution results, retained artifact bundles, compare digest publication, and freeze-review readiness.

## Authoritative packet families
| Packet family | Purpose |
|---|---|
| `packet.cert.result.v1` | terminal success or terminal failure for one certification pack |
| `packet.cert.partial.v1` | progress-bearing partial result for long-running pack execution |
| `packet.cert.artifact.bundle.v1` | retained artifact references for one certification run |
| `packet.cert.compare.digest.v1` | compare-ready digest linking baseline, failed run, and recovery run |
| `packet.cert.freeze.signal.v1` | explicit freeze-block, freeze-ready, or signoff escalation signal |

## Field-level schema table
| Field | Required | Notes |
|---|---|---|
| `pack_id` | yes | canonical pack id from root pack registry |
| `pack_revision` | yes | pack content revision |
| `scenario_id` | yes | exact scenario row |
| `result_state` | yes | `partial`, `terminal_success`, `retryable_failure`, `terminal_failure` |
| `terminal_code` | conditional | mandatory on terminal states |
| `first_failure_code` | conditional | mandatory on failure states |
| `artifact_bundle_ref` | yes | authoritative retained artifact bundle |
| `baseline_ref` | conditional | mandatory on compare-bearing or freeze-bearing output |
| `failed_run_ref` | conditional | mandatory on recovery or retry-bearing output |
| `recovery_run_ref` | conditional | mandatory once a recovery rerun exists |
| `focus_target_id` | yes | next legal operator focus |
| `next_action_id` | yes | next legal action or `action.none` |
| `freeze_relevance` | yes | `none`, `review_required`, `freeze_blocker`, or `signoff_ready` |
| `evidence_row_ref` | conditional | mandatory once evidence append has happened |

## Result and code registry
### Terminal success codes
- `cert.pass`
- `cert.pass_with_degrade`
- `cert.pass_with_retained_warning`

### Retryable failure codes
- `cert.retry.threshold_spike`
- `cert.retry.transient_attachment_gap`
- `cert.retry.route_cache_stale`

### Terminal failure codes
- `cert.fail.pack_schema_gap`
- `cert.fail.baseline_missing`
- `cert.fail.compare_window_gap`
- `cert.fail.freeze_blocker_present`

## Compare and replay payload contract
Any certification packet that advertises compare completeness must carry:
- `baseline_ref`
- `failed_run_ref`
- `recovery_run_ref` when recovery exists
- `compare_mode_id`
- `digest_hash`
- `window_start_tick`
- `window_end_tick`
- `degrade_step_id` when degradation occurred

A certification consumer must reject compare publication when the triplet is incomplete.

## Compatibility and version law
- `pack_id` and `scenario_id` are semantic keys and may not be reinterpreted by downstream tooling;
- any packet revision that changes `freeze_relevance`, `terminal_code`, or `artifact_bundle_ref` semantics requires a major compatibility bump;
- artifact references may gain more fields, but may not stop resolving to one retained bundle.

## Evidence duties
Certification result publication must preserve:
- one retained artifact bundle;
- one compare digest or an explicit compare-not-legal reason;
- one focus target;
- one next legal recovery action when terminal success is absent.

## Failure and focus preservation law
- no certification result may terminate without `next_action_id`;
- no freeze-bearing result may omit `artifact_bundle_ref`;
- no retryable failure may erase `failed_run_ref`;
- no signoff-bearing result may hide a surviving degrade step.
