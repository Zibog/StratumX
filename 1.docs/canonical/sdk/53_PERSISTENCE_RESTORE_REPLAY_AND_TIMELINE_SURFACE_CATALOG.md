# Persistence Restore Replay And Timeline Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact typed bridge for this heavy-domain family.
This file owns public packet families, field tables, code registries, compatibility law, replay/compare payload contracts, and evidence duties.

## Exact packet families
| Packet family | Mandatory payload meaning |
|---|---|
| `packet.timeline.restore_triplet.v4` | restore anchor, baseline/failed/recovery ids, replay horizon |
| `packet.timeline.anchor_verdict.v3` | anchor legality, restore scope, next action |
| `packet.timeline.capture_manifest.v3` | retained artifacts for replay/certification |

## Field-level schema table
| Field | Required |
|---|---|
| `anchor_id` | yes |
| `restore_scope_id` | yes |
| `replay_window_id` | yes |
| `baseline_id` | yes |
| `failed_run_id` | yes |
| `recovery_run_id` | yes |
| `artifact_ref` | yes |
| `trace_ref` | yes |
| `first_failure_code` | yes |
| `next_action_id` | yes |

## Domain enum and failure-code registries
- restore_scope: local, zone, global
- timeline_state: baseline, failed_run, recovery_run
- artifact_class: trace, compare, pack, bundle

Failure codes:
- `timeline.anchor_missing`
- `timeline.triplet_incomplete`
- `timeline.restore_scope_illegal`

## Replay payload contract
all restore payloads must carry triplet ids and stable anchor ids.

## Compare payload contract
compare payloads must carry replay_window_id and compare_mode_id.

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
