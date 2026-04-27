# Navigation Pathfinding And Traversal Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define generic navigation truth for pathing, cover, ladders, doors, climb, destructible traversal and crowd/tactical interaction.

## Exact truth objects
| Truth object | Role | Authoritative fields | Publication scope |
|---|---|---|---|
| `traversal_surface_profile` | Authored traversable surface row | `surface_class`, `cost_band`, `door_ladder_climb_flags`, `destruction_response` | nav + tactics + editor |
| `cover_traversal_graph` | Cover + route graph | `node_class`, `exposure_band`, `fallback_link`, `destruction_hook` | nav + tactics |
| `crowd_pressure_state` | Crowd and congestion truth | `density_band`, `queue_policy`, `degrade_rung`, `hazard_mask` | nav + performance |
| `path_rebuild_anchor` | Path rebuild legality anchor | `cause_event`, `affected_tiles`, `baseline_ref`, `failed_run_id` | nav + recovery |

## Exact phase order
| Phase | Input | Output | Illegal shortcut |
|---|---|---|---|
| `author_surface_profile` | canonical material/surface request | traversal surface row | path cost authored outside registry |
| `bind_graph` | surface rows + topology | cover/traversal graph | graph build without destruction hook |
| `simulate_route_or_rebuild` | route query or world delta | route result or rebuild delta | silent rebuild on destruction |
| `publish_navigation_explainability` | route/rebuild result | diagnostics event + compare payload | route answer without blocker trace |

## Coupling boundaries
| Boundary | Allowed through | Forbidden | Reason |
|---|---|---|---|
| nav -> tactics | `packet.nav.bind_cover_graph.v1` | tactic doctrine hardcoding cover | cover legality belongs to shared graph |
| nav -> destruction | `event.nav.rebuild_required.v1` | stale traversal after collapse | destruction affects route legality |
| nav -> causality | `event.nav.route_reason.v1` | path result without reason chain | operator must explain route choice |

## Failure and denial families
| Family | Meaning | Retryable | Required artifact / trace |
|---|---|---|---|
| `fail.surface_profile_missing` | surface row absent | no | `artifact.nav.surface_registry`, `trace.nav.surface_lookup` |
| `fail.graph_binding_incomplete` | graph lacks mandatory hooks | yes | `artifact.nav.graph`, `trace.nav.bind` |
| `fail.rebuild_anchor_missing` | rebuild required but no legal anchor | yes | `artifact.nav.recovery`, `trace.nav.rebuild` |

## Resource envelope and degrade law
| Axis | Nominal law | Degrade rung | May never be faked |
|---|---|---|---|
| CPU | path search bounded by tier and affected tiles | drop long-horizon crowd refinement | door/ladders/climb legality |
| GPU | navigation has no truth GPU dependency | presentation-only overlays may thin | route truth |
| RAM | graph tiers hot/warm/cold | demote historical congestion first | active blocker + rebuild anchors |
| Disk/IO | rebuild bundles are tile-scoped | delay archival route captures | baseline triplet |

## Publication and evidence obligations
| Event / artifact | Publisher | Required payload | Consumer |
|---|---|---|---|
| `event.nav.route_published.v1` | engine/72 | `route_id`, `surface_class`, `degrade_rung`, `trace_ref`, `artifact_ref` | sdk/71 + editor/76 |
| `artifact.nav.route_triplet` | tooling | baseline/failed/recovery, path digest, coverage digest | editor/105 + editor/109 |

## Legal recovery actions
| Failure family | Legal recovery | Required anchor | Next legal focus |
|---|---|---|---|
| `fail.surface_profile_missing` | author/bind traversal surface | surface profile baseline | `editor/76` |
| `fail.graph_binding_incomplete` | rebind graph and simulate | graph artifact | `editor/76_then_100` |
| `fail.rebuild_anchor_missing` | recover failed-run and rerun compare | failed-run id + baseline | `editor/76_then_105` |



## Exact editor entrypoints
- `btn.nav.author_traversal_profile`
- `btn.nav.bind_cover_policy`
- `btn.nav.inspect_route_legality`
- `btn.nav.simulate_path_preview`
- `btn.nav.compare_route_triplet`
- `btn.nav.capture_route_evidence`
- `btn.nav.recover_nav_baseline`
- `btn.nav.certify_nav_pack`

## Phase-3 proof slices
- navmesh and traversal legality rebuild after destruction without hidden teleports;
- blocked reasons remain visible to the operator and stable across compare/recover;
- every navigation rebuild exports one reason fragment to `engine/76`.

## Required publications
- `event.nav.route_or_blocker_delta.v1` with `route_graph_ref`, `blocker_code`, `rebuild_anchor`, `trace_ref`, `artifact_ref`;
- `artifact.nav.route.triplet` with baseline/failed/recovery and cover traversal digest.


## Status
`document_gold / doc_closed_impl_open`
