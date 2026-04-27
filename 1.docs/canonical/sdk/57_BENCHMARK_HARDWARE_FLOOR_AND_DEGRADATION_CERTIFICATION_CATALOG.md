# Benchmark Hardware Floor And Degradation Certification Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact typed bridge for this heavy-domain family.
This file owns public packet families, field tables, code registries, compatibility law, replay/compare payload contracts, and evidence duties.

## Exact packet families
| Packet family | Mandatory payload meaning |
|---|---|
| `packet.floor.pressure_snapshot.v3` | axis buckets, threshold rows, first blocking code |
| `packet.floor.degrade_transition.v3` | old step, new step, legality verdict, trigger row |
| `packet.floor.recovery_step.v3` | active axis, current step, next legal recovery action, baseline pointer |
| `packet.floor.pack_result.v2` | pack id, pass/fail, retained artifact manifest, freeze posture |

## Field-level schema table
| Field | Required |
|---|---|
| `pack_id` | yes |
| `axis` | yes |
| `threshold_row_id` | yes |
| `active_ladder_step` | yes |
| `baseline_pointer` | yes |
| `first_failure_code` | yes |
| `evidence_duty` | yes |
| `next_action_id` | yes |
| `focus_target_id` | yes |

## Domain enum and failure-code registries
- axis: cpu, gpu, ram, disk
- bucket: green, yellow, orange, red
- failure_family: floor.*, pressure.*
- pack_result: pass, fail, blocked

Failure codes:
- `floor.axis_red_unresolved`
- `floor.retained_artifact_gap`
- `pressure.threshold_row_missing`

## Replay payload contract
replay-bearing payloads must include stable ids, compare window id, and baseline pointer.

## Compare payload contract
compare-bearing payloads must include axis raw values, normalized values, and threshold row ids.

## Compatibility and version rules
- any field-order, semantics, enum expansion, or lifecycle change that affects compare or replay meaning requires a version bump;
- normalization may compact payload shape only if the raw values, first failure code, next action id, and focus target id remain present;
- consumers may not infer unnamed payloads, unnamed artifacts, or unnamed codes.

## Evidence duties
- every certification-bearing or compare-bearing packet must preserve retained artifact references;
- every terminal failure must preserve first failure code and next legal action;
- every terminal success that participates in certification must preserve compare ids and evidence posture;
- lifecycle semantics must agree with sdk `77`.

## Current posture
`document_gold / doc_closed_impl_open`
