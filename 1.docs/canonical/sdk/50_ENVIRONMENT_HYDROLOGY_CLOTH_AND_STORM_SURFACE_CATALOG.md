# Environment Hydrology Cloth And Storm Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact typed bridge for this heavy-domain family.
This file owns public packet families, field tables, code registries, compatibility law, replay/compare payload contracts, and evidence duties.

## Exact packet families
| Packet family | Mandatory payload meaning |
|---|---|
| `packet.hydro.restore_triplet.v4` | mass ledger, leak graph, restore anchor, triplet ids |
| `packet.fire.weather.verify.v4` | thermal cell bands, wind bucket, smoke band, coupling verdict |
| `packet.softsurface.response_review.v3` | solver rung, wetness class, contact band, recovery anchor |
| `packet.environment.compare_triplet.v3` | baseline/failed/recovery ids for environment heavy domains |

## Field-level schema table
| Field | Required |
|---|---|
| `scope_id` | yes |
| `container_or_cell_id` | yes |
| `mass_delta` | yes |
| `leak_topology_id` | yes |
| `evaporation_tier` | yes |
| `thermal_bucket` | yes |
| `wind_bucket` | yes |
| `smoke_band` | yes |
| `solver_rung` | yes |
| `wetness_class` | yes |
| `contact_band` | yes |
| `baseline_id` | yes |
| `failed_run_id` | yes |
| `recovery_run_id` | yes |
| `first_failure_code` | yes |

## Domain enum and failure-code registries
- evaporation_tier: low, medium, high, extreme
- thermal_bucket: ember, ignite, burn, flare
- solver_rung: r0, r1, r2, r3
- wetness_class: dry, damp, wet, saturated

Failure codes:
- `hydro.mass.nonconserve`
- `fire.thermal.coupling_break`
- `soft.solver.diverge`

## Replay payload contract
replay-bearing payloads must carry one replay_window_id and one recovery anchor id.

## Compare payload contract
compare payloads must expose raw and normalized mass/thermal/response values for tolerance review.

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
