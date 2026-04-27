# Heavy Domain Authoring Transaction Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Publish transaction families for heavy-domain authoring.

## Request packet families

| Packet family | Required fields | Primary result family |
|---|---|---|
| `packet.tx.terrain_deformation_author.v1` | `request_id`, `terrain_ref`, `deformation_class`, `dirty_volume_ref`, `validation_gate_ids`, `recovery_anchor_ref` | `packet.tx.result.terrain_deformation.v1` |
| `packet.tx.destruction_topology_author.v1` | `request_id`, `target_topology_ref`, `support_group_ids`, `structural_class`, `validation_gate_ids`, `recovery_anchor_ref` | `packet.tx.result.destruction_topology.v1` |
| `packet.tx.weather_front_inject.v1` | `request_id`, `front_seed_ref`, `region_scope`, `arrival_window`, `validation_gate_ids`, `recovery_anchor_ref` | `packet.tx.result.weather_front_inject.v1` |
| `packet.tx.hydrology_source_edit.v1` | `request_id`, `container_or_source_ref`, `fluid_inventory_ref`, `leak_geometry_ref`, `validation_gate_ids`, `recovery_anchor_ref` | `packet.tx.result.hydrology_source_edit.v1` |
| `packet.tx.tactical_sandbox_edit.v1` | `request_id`, `squad_or_cover_scope_ref`, `shared_intent_code`, `reservation_refs`, `validation_gate_ids`, `recovery_anchor_ref` | `packet.tx.result.tactical_sandbox_edit.v1` |
| `packet.tx.species_wound_rule_edit.v1` | `request_id`, `species_topology_class`, `wound_rule_bundle_ref`, `validation_gate_ids`, `recovery_anchor_ref` | `packet.tx.result.species_wound_rule_edit.v1` |
| `packet.tx.groom_setup_edit.v1` | `request_id`, `groom_asset_ref`, `representation_rung`, `coverage_mask_ref`, `validation_gate_ids`, `recovery_anchor_ref` | `packet.tx.result.groom_setup_edit.v1` |

## Result packet law
Every authoring transaction result packet must include:
- `request_id`
- `terminal_state`
- `first_blocker_code`
- `invalidation_set_ref`
- `saved_artifact_posture`
- `undo_legality`
- `redo_legality`
- `focus_target_surface_id`
- `reason_trace_ref`

## Failure families
- `TX_PRECOND_*`
- `TX_VALID_*`
- `TX_APPLY_*`
- `TX_UNDO_*`
- `TX_SAVE_*`
- `TX_FOCUS_*`

## Ownership law
- request packets may be created by editor/tooling, but only routed through lawful tooling families;
- terminal mutation authority remains below sdk;
- sdk owns the public request/result contract, not the mutation itself.
