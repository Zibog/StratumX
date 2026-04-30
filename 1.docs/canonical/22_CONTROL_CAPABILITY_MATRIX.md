# Control Capability Matrix

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Map capability families to the active production contour using one vocabulary: truth owner, owner surfaces, button namespace, canonical pack ids, and current posture.

## Matrix
| Capability family | Engine truth owners | Editor production surfaces | Button namespace | Canonical pack ids | Current posture |
|---|---|---|---|---|---|
| input and action-map resolution | `engine/59` | `editor/63 + editor/64` | control command rows in `editor/110` | `pack.frame_budget_triplet` | `doc_closed_impl_open` |
| project ladder to build and launch | `engine/84 + engine/97` | `editor/82 + editor/106 + editor/109` | project + freeze command rows in `editor/110` | `pack.release_freeze_signoff` | `doc_closed_impl_open` |
| evidence and certification control | `engine/84 + engine/97` | `editor/103 + editor/105` | certification + evidence command rows in `editor/110` | `pack.certification_scenario_review + pack.evidence_triplet_append` | `doc_closed_impl_open` |

## Law
These matrices may not invent local namespaces, local pack aliases, or local route names outside root `71`, `72`, `74`, `76`, `81`, and `editor/110`.
