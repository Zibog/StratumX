# Navigation Pathfinding And Traversal Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact typed bridge for path requests, dynamic blockers, reroute causes, traversal legality, and cover-aware route publication.

## Exact packet families
- `packet.nav.route_result.v2` — route choice, path digest, compare window;
- `packet.nav.blocker_delta.v1` — dynamic blocker set delta and cause code;
- `packet.nav.reroute_reason.v1` — reroute cause, recovery action id, baseline pointer;

## Field-level schema table
| Field | Meaning | Required |
|---|---|---|
| `stable_slice_id` | route slice identity | yes |
| `scope_tag` | actor, squad, convoy, region | yes |
| `tier_tag` | hot, warm, restore | yes |
| `compare_window_id` | route compare window | yes |
| `tolerance_class` | nav.route, nav.restore | yes |
| `baseline_pointer` | last-good route baseline | conditional |
| `artifact_pointer` | retained artifact pointer | yes |
| `first_failure_code` | first route failure family | conditional |

## Enum, code, and registry obligations
- `scope_tag`: actor, squad, convoy, region;
- `compare_class`: fast_route, certification_route, restore_route;
- `evidence_duty`: retain_compare, retain_baseline, retain_recovery;
- `normalization_class`: route_exact, blocker_digest;

## Compatibility and normalization law
- normalization may compact blocker lists but never change legal route choice semantics;
- compatibility break requires version bump for reroute or blocker semantics changes;
- packet consumers may not teleport path truth by normalizing away reroute cause;

## Replay and compare payload contracts
- replay payload must include route id, compare window, reroute cause, and baseline pointer when restore-bearing;
- compare payload must expose chosen route and blocker digest;

## Evidence duties
- retain route compare and blocker digest artifacts for certification;
- retain failed-run artifact on route or cover-link failure;
- never drop reroute cause codes;

## Current posture
`document_gold / doc_closed_impl_open`
