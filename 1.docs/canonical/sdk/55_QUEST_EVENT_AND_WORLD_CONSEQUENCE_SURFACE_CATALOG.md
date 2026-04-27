# Quest Event And World Consequence Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact typed bridge for quest state, event transitions, systemic consequences, checkpoint access, and recovery publication.

## Exact packet families
- `packet.quest.transition_result.v2` — quest/event transition, precondition verdict, compare window;
- `packet.quest.consequence_delta.v2` — world consequence delta, checkpoint access state, baseline pointer;
- `packet.quest.recovery_step.v1` — recovery action id, retained baseline pointer, artifact pointer;

## Field-level schema table
| Field | Meaning | Required |
|---|---|---|
| `stable_slice_id` | quest/consequence slice identity | yes |
| `scope_tag` | quest, event, checkpoint, world | yes |
| `tier_tag` | hot, warm, restore | yes |
| `compare_window_id` | quest compare window | yes |
| `tolerance_class` | quest.transition, quest.restore | yes |
| `baseline_pointer` | last-good quest baseline | conditional |
| `artifact_pointer` | retained artifact pointer | yes |
| `first_failure_code` | first quest failure family | conditional |

## Enum, code, and registry obligations
- `scope_tag`: quest, event, checkpoint, world;
- `compare_class`: fast_transition, certification_consequence, restore_checkpoint;
- `evidence_duty`: retain_compare, retain_baseline, retain_recovery;
- `normalization_class`: transition_exact, consequence_digest;

## Compatibility and normalization law
- normalization may compact explanatory payloads but never change consequence semantics;
- compatibility break requires version bump for any consequence or checkpoint access semantics change;
- packet consumers may not hide hidden-consequence failures by normalization;

## Replay and compare payload contracts
- replay payload must include transition id, compare window, checkpoint access state, and baseline pointer when restore-bearing;
- compare payload must expose precondition verdict and consequence digest;

## Evidence duties
- retain consequence compare and recovery action for each certification run;
- retain failed-run artifact when hidden-consequence or precondition failures appear;
- never suppress first failure codes;

## Current posture
`document_gold / doc_closed_impl_open`
