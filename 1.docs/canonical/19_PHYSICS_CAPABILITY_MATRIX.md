# Physics Capability Matrix

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Map physics capability families to the active production contour using one vocabulary: truth owner, owner surfaces, button namespace, canonical pack ids, and shared response classes defined by root `88`.

## Matrix
| Capability family | Shared response classes | Engine truth owners | Editor production surfaces | Button namespace | Canonical pack ids | Current posture |
|---|---|---|---|---|---|---|
| hydrology and persistence | hydrology, thermal/wet response, persistence consequence | `engine/61 + engine/73` | `editor/66 + editor/77` | hydrology + timeline command rows in `editor/110` | `pack.hydrology_persistence` | `doc_closed_impl_open` |
| fire, storm, smoke, wetness | thermal/burn/wet response, media transport | `engine/49 + engine/62 + engine/69` | `editor/67 + editor/74` | fire + storm + world-floor command rows in `editor/110` | `pack.fire_weather_smoke + pack.storm_long_range_visibility` | `doc_closed_impl_open` |
| cloth, fur, soft contact | soft response, contact response | `engine/63 + engine/70` | `editor/68 + editor/75` | soft-surface + animation command rows in `editor/110` | `pack.fur_cloth_weather` | `doc_closed_impl_open` |
| destruction, terrain blast, collapse aftermath | rigid response, fracture/collapse response, traversal consequence | `engine/50 + engine/65 + engine/69` | `editor/58 + editor/59 + editor/70 + editor/74` | terrain + structural + tactics + world-floor command rows in `editor/110` | `pack.terrain_crater_truth + pack.structure_cascade_aftermath + pack.secondary_collision_aftermath + pack.terrain_blast_degrade` | `doc_closed_impl_open` |
| ballistics, penetration, wounds | ballistic penetration response, rigid response, fracture response | `engine/48 + engine/67 + engine/71` | `editor/55 + editor/72` | wound and ballistic command rows in `editor/110` | `pack.ballistics_wound_trace` | `doc_closed_impl_open` |
| navigation and traversal legality | traversal and navigation consequence | `engine/72` | `editor/76` | navigation command rows in `editor/110` | `pack.navigation_traversal_cover` | `doc_closed_impl_open` |

## Law
These matrices may not invent local namespaces, local pack aliases, or local route names outside root `71`, `72`, `74`, `76`, `81`, `88`, and `editor/110`.
