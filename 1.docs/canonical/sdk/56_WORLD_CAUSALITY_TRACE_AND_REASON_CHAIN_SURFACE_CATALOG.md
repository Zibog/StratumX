# World Causality Trace And Reason Chain Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact typed bridge for cause-chain publication, provenance fragments, operator-facing explanations, and compare/recovery routing.

## Exact packet families
- `packet.trace.reason_chain.v2` — reason chain id, fragment list, compare window;
- `packet.trace.provenance_gap.v1` — missing provenance, denied explanation scope, recovery action;
- `packet.trace.compare_digest.v1` — compare digest pointer, baseline pointer, artifact pointer;

## Field-level schema table
| Field | Meaning | Required |
|---|---|---|
| `stable_slice_id` | trace slice identity | yes |
| `scope_tag` | incident, entity, squad, settlement, world | yes |
| `tier_tag` | hot, warm, restore, review | yes |
| `compare_window_id` | trace compare window | yes |
| `tolerance_class` | trace.chain, trace.restore | yes |
| `baseline_pointer` | last-good explanation baseline | conditional |
| `artifact_pointer` | retained artifact pointer | yes |
| `first_failure_code` | first trace failure family | conditional |

## Enum, code, and registry obligations
- `scope_tag`: incident, entity, squad, settlement, world;
- `compare_class`: fast_trace, certification_trace, restore_trace;
- `evidence_duty`: retain_chain, retain_compare, retain_baseline, retain_recovery;
- `normalization_class`: provenance_exact, explanation_digest;

## Compatibility and normalization law
- normalization may compact explanatory wording but never fabricate or remove authoritative fragments;
- compatibility break requires version bump for fragment semantics or provenance requirements changes;
- packet consumers may not close provenance gaps with synthetic fragments;

## Replay and compare payload contracts
- replay payload must include reason chain id, compare window, baseline pointer when restore-bearing, and provenance completeness flag;
- compare payload must expose raw fragment ids and normalized digest;

## Evidence duties
- retain chain artifact, compare digest, and recovery action for every certification review;
- retain failed-run artifact when provenance gaps appear;
- never suppress denied explanation scope or first failure code;

## Current posture
`document_gold / doc_closed_impl_open`
