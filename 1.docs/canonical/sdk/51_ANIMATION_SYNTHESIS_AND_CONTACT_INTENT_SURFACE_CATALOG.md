# Animation Synthesis And Contact Intent Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact typed bridge for micro-motion synthesis, contact intent, procedural pose deltas, and contact confidence publication.

## Exact packet families
- `packet.anim.contact_intent.v2` — contact objective, solver state, compare window;
- `packet.anim.micro_motion_digest.v2` — procedural delta digest, blend commit id, tolerance class;
- `packet.anim.restore_anchor.v1` — restore-bearing motion anchor and retained baseline pointer;

## Field-level schema table
| Field | Meaning | Required |
|---|---|---|
| `stable_slice_id` | animation slice identity | yes |
| `scope_tag` | actor, limb, contact_pair, scene | yes |
| `tier_tag` | hot, warm, restore | yes |
| `compare_window_id` | animation/contact window id | yes |
| `tolerance_class` | anim.contact, anim.restore | yes |
| `baseline_pointer` | last-good motion baseline | conditional |
| `artifact_pointer` | retained artifact pointer | yes |
| `first_failure_code` | first authoritative animation failure | conditional |

## Enum, code, and registry obligations
- `scope_tag`: actor, limb, contact_pair, scene;
- `compare_class`: fast_contact, certification_contact, restore_motion;
- `evidence_duty`: retain_compare, retain_baseline, retain_recovery;
- `normalization_class`: pose_exact, micro_motion_digest;

## Compatibility and normalization law
- normalization may compact secondary-motion detail but never alter contact legality;
- compatibility break requires version bump for any contact or blend semantics change;
- packet consumers may not treat sampled preview as authoritative contact;

## Replay and compare payload contracts
- replay payload must include blend commit id, compare window, and baseline pointer when restore-bearing;
- compare payload must expose raw contact confidence and normalized digest;

## Evidence duties
- retain contact compare and recovery action for each certification run;
- retain baseline before any persistent blended commit;
- never suppress first failure code from downstream compare surfaces;

## Current posture
`document_gold / doc_closed_impl_open`
