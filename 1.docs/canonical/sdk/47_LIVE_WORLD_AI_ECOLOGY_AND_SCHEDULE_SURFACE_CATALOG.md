# Live World AI Ecology And Schedule Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact typed bridge for this heavy-domain family.
This file owns public packet families, field tables, code registries, compatibility law, replay/compare payload contracts, and evidence duties.

## Exact packet families
| Packet family | Mandatory payload meaning |
|---|---|
| `packet.society.schedule_audit.v4` | schedule legality, need pressure, crime bucket, base-formation digest |
| `packet.tactics.cover_review.v4` | cover graph digest, suppression bands, rollback anchor |
| `packet.ecology.route_audit.v4` | migration corridor rows, predation pressure, recovery tier |
| `packet.liveworld.compare_triplet.v3` | baseline/failed/recovery ids for society+tactics+ecology |

## Field-level schema table
| Field | Required |
|---|---|
| `scope_id` | yes |
| `time_band_id` | yes |
| `need_vector_id` | yes |
| `crime_bucket` | yes |
| `faction_zone_id` | yes |
| `cover_graph_id` | yes |
| `suppression_band` | yes |
| `corridor_edge_id` | yes |
| `predation_pressure` | yes |
| `rollback_anchor_id` | yes |
| `baseline_id` | yes |
| `failed_run_id` | yes |
| `recovery_run_id` | yes |
| `first_failure_code` | yes |

## Domain enum and failure-code registries
- society_scope: district, zone, base
- crime_bucket: low, medium, high, crisis
- suppression_band: green, amber, red
- migration_tier: local, regional, long_range

Failure codes:
- `society.schedule.conflict_unresolved`
- `society.crime.band_missing`
- `tactics.cover.graph_invalid`
- `ecology.route.edge_break`

## Replay payload contract
replay-bearing payloads must include stable scope id, baseline/failed/recovery ids, replay_window_id, and first failure code.

## Compare payload contract
compare payloads must include raw and normalized pressure values plus declared tolerance profile from root `67`.

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
