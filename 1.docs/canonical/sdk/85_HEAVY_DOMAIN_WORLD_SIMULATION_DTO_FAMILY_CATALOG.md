# Heavy Domain World Simulation DTO Family Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define exact public DTO families for dream-scene heavy domains.

## Packet version law
- every family below is public and versioned;
- version bumps are required for breaking field or semantic changes;
- stable ids are mandatory for every emitted public object;
- domain-local labels are forbidden as public identifiers.

## Packet families

| Packet family | Required fields | Ownership | Primary consumers |
|---|---|---|---|
| `packet.destruction.topology_delta.v1` | `topology_ref`, `support_group_ids`, `breach_class`, `collapse_eligibility`, `fragment_release_refs`, `reason_trace_ref` | engine `110/117` | editor `120`, tooling `50/85/86` |
| `packet.terrain.deformation_slice.v1` | `terrain_ref`, `deformation_class`, `dirty_volume_ref`, `removed_mass`, `deposited_mass`, `nav_cover_delta_ref` | engine `111` | editor `119`, tooling `43/85/86` |
| `packet.hydrology.container_state.v1` | `container_ref`, `fluid_inventory_ref`, `fill_ratio`, `leak_geometry_ref`, `inflow_rate`, `outflow_rate`, `contamination_class`, `freeze_state` | engine `112` | editor `121`, tooling `48/55/85/86` |
| `packet.climate.front_state.v1` | `front_ref`, `front_lifecycle_state`, `region_scope`, `arrival_window`, `precipitation_class`, `lightning_event_refs`, `lunar_phase_code` | engine `114–115` | editor `122`, tooling `48/84/85/86` |
| `packet.tactics.squad_intent.v1` | `squad_ref`, `shared_intent_code`, `role_assignment_refs`, `suppression_score`, `flank_reservation_refs`, `cover_validity_digest_ref` | engine `121` | editor `123`, tooling `49/85/86` |
| `packet.society.delta.v1` | `agent_or_group_ref`, `need_vector_ref`, `resource_delta_ref`, `status_delta_ref`, `crime_event_ref`, `reputation_delta_ref` | engine `120` | editor `123`, tooling `49/57/85/86` |
| `packet.wound.layered_result.v1` | `body_ref`, `species_topology_class`, `layer_entry_ref`, `organ_zone_ref`, `bone_hit_flag`, `survivability_verdict`, `gore_legality_code` | engine `119` | editor `124`, tooling `50/55/85/86` |
| `packet.microgeometry.coverage_state.v1` | `asset_ref`, `representation_rung`, `coverage_mask_ref`, `wind_response_class`, `wetness_state`, `char_state`, `budget_code` | engine `127` | editor `125`, tooling `48/53/83/85/86` |
| `packet.photoreal.fallback_report.v1` | `scene_class`, `hardware_profile`, `lighting_rung`, `shadow_rung`, `volumetric_rung`, `reflection_rung`, `first_blocker_code` | engine `126` | editor `126`, tooling `52/59/84/86` |

## Delivery guarantees

| Class | Guarantee |
|---|---|
| live-per-frame packets | best-effort within declared degrade posture; silent drop is forbidden |
| event-driven packets | exactly-once retained ledger emission per authoritative event id |
| snapshot packets | explicit request/response or checkpoint emission |
| certification packets | immutable retained artifact emission |

## Failure families
- `PKT_COMPAT_*`: incompatible packet version or semantic mismatch;
- `PKT_OWN_*`: non-authoritative emitter;
- `PKT_FIELD_*`: required field missing or malformed;
- `PKT_SCOPE_*`: illegal scope/window for the packet family;
- `PKT_RATE_*`: cadence exceeded the allowed policy.

## Consumer law
Editor and tooling may consume only declared packet families, never engine-private structs.
