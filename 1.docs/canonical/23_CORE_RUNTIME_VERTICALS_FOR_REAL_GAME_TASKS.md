# Core Runtime Verticals For Real Game Tasks

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Map capability families to the active production contour using one vocabulary: truth owner, owner surfaces, button namespace, canonical pack ids, and current posture.

## Matrix
| Capability family | Engine truth owners | Editor production surfaces | Button namespace | Canonical pack ids | Current posture |
|---|---|---|---|---|---|
| bootstrap -> import -> bind | `engine/84 + engine/100 + engine/102` | `editor/82 + editor/107` | project + content command rows in `editor/110` | `pack.release_freeze_signoff` | `doc_closed_impl_open` |
| world -> gameplay -> heavy simulation | `engine/45 + engine/47–76` | `editor/108 + editor/58–81` | world/gameplay/heavy-domain command rows in `editor/110` | `domain canonical packs from root 81` | `doc_closed_impl_open` |
| systemic gameplay: nav / inventory / quest / reason | `engine/72 + engine/74 + engine/75 + engine/76` | `editor/76 + editor/78 + editor/79 + editor/80` | nav + inventory + quest + reason command rows in `editor/110` | `world/gameplay canonical packs from root 81` | `doc_closed_impl_open` |
| presentation/runtime -> cert / evidence / freeze | `engine/86–98` | `editor/90–98 + editor/103 + editor/105 + editor/109` | presentation + certification + evidence + freeze command rows in `editor/110` | `presentation/runtime canonical packs from root 81` | `doc_closed_impl_open` |

## Law
These matrices may not invent local namespaces, local pack aliases, or local route names outside root `71`, `72`, `74`, `76`, `81`, and `editor/110`.
