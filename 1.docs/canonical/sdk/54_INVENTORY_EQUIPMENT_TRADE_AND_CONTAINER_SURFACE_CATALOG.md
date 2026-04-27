# Inventory Equipment Trade And Container Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact typed bridge for item identity, container legality, equipment state, loot transitions, and trade publication.

## Exact packet families
- `packet.inventory.item_state.v2` — item id, slot/container state, compare window;
- `packet.inventory.trade_delta.v1` — trade pressure delta, scarcity band, artifact pointer;
- `packet.inventory.restore_anchor.v1` — restore-bearing inventory anchor and recovery action;

## Field-level schema table
| Field | Meaning | Required |
|---|---|---|
| `stable_slice_id` | inventory slice identity | yes |
| `scope_tag` | item, container, actor, market | yes |
| `tier_tag` | hot, warm, restore | yes |
| `compare_window_id` | inventory compare window | yes |
| `tolerance_class` | inventory.identity, inventory.trade | yes |
| `baseline_pointer` | last-good inventory baseline | conditional |
| `artifact_pointer` | retained artifact pointer | yes |
| `first_failure_code` | first inventory failure family | conditional |

## Enum, code, and registry obligations
- `scope_tag`: item, container, actor, market;
- `compare_class`: fast_inventory, certification_inventory, restore_inventory;
- `evidence_duty`: retain_compare, retain_baseline, retain_recovery;
- `normalization_class`: item_exact, trade_digest;

## Compatibility and normalization law
- normalization may bucket non-critical trade deltas but never alter item identity or slot legality;
- compatibility break requires version bump for any identity or slot semantics change;
- packet consumers may not merge items by normalization alone;

## Replay and compare payload contracts
- replay payload must include item identity, compare window, and baseline pointer when restore-bearing;
- compare payload must expose raw identity and normalized trade digest;

## Evidence duties
- retain item identity compare and recovery action artifacts for certification;
- retain baseline before any persistent split/merge action;
- never suppress first failure code for slot/container illegality;

## Current posture
`document_gold / doc_closed_impl_open`
