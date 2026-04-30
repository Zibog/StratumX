# Technology Schema Field Registry Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
This registry freezes mandatory field ids that appear across command, routing, packet, and artifact contracts.

## Mandatory cross-layer fields
| Field id | Law |
|---|---|
| `request_id` | stable through retries and recovery |
| `route_id` | exact tooling route id |
| `packet_family` | exact sdk family id |
| `truth_owner_id` | exact engine owner id |
| `focus_target_id` | exact next legal focus |
| `next_action_id` | exact next legal action |
| `artifact_ref` | retained artifact reference when any artifact exists |
| `trace_ref` | retained trace reference when diagnostics or compare depend on trace |
| `baseline_ref` | baseline pointer for compare-bearing or freeze-bearing output |
| `failed_run_ref` | failed-run pointer for retry and recovery |
| `compare_mode_id` | canonical compare mode id |
| `capture_mode_id` | canonical capture mode id |
| `first_failure_code` | first blocker or failure code |
| `terminal_code` | terminal success or terminal failure code |
| `schema_revision` | field-law revision for compatibility |
| `bridge_version` | packet or artifact bridge version |
| `world_ref` | stable world identity for world-, terrain-, and sky-bearing commands |
| `terrain_ref` | stable terrain identity for terrain-bearing commands |
| `chunk_region_ref` | exact chunk or dirty-region scope for terrain mutations |
| `material_archetype_ref` | stable archetype identity |
| `surface_family_ref` | stable surface-family identity |
| `response_profile_ref` | stable response-profile identity |
| `texture_stack_ref` | stable texture stack identity |
| `microdetail_profile_ref` | stable microdetail binding identity |
| `weather_modulation_profile_ref` | stable weather modulation binding identity |
| `damage_visual_profile_ref` | stable damage visual binding identity |
| `aftermath_overlay_profile_ref` | stable aftermath overlay identity |
| `terrain_layer_weight_map_ref` | stable terrain layer-weight map identity |
| `biome_overlay_map_ref` | stable biome-overlay map identity |
| `material_instance_ref` | stable material instance stack identity |
| `blend_policy_ref` | stable blend-policy identity when terrain or layered surfaces are mutated |
| `biome_overlay_ref` | stable biome or litter overlay identity when present |
| `consequence_tier_ref` | stable sleep/wake/consequence tier identity when response is tiered |
| `sleep_wake_policy_ref` | stable cheap-consequence sleep/wake policy identity |
| `source_lineage_ref` | import or authoring lineage reference for save/import/save-as actions |

## Law
- a packet or artifact participating in truth-to-editor closure is invalid when required fields are absent;
- silent field synthesis across layers is forbidden;
- field labels may vary by UI, field ids may not;
- a terrain or material command without `world_ref` plus the correct material/terrain refs is invalid even if the UI can guess the target.


## Base-shell and advanced authoring fields
| Field id | Law |
|---|---|
| `visibility_class` | button visibility contract across shell, playbook, and manifest |
| `owner_surface_id` | authoritative editor surface that owns the action |
| `dependency_gate_id` | canonical gate that must resolve before activation |
| `validation_gate_ids` | ordered validation gates checked before commit or retained preview |
| `derived_invalidation_set` | named caches, indices, or queues invalidated by the action |
| `viewport_publication_set` | required viewport overlays or repaint classes after the action |
| `shell_publication_set` | required inspector, outliner, and status-bar publications after the action |
| `persistence_posture` | immediate-save, pending-save, or transient-artifact law |
| `recovery_anchor_ref` | exact rollback or return anchor created or used by the action |
| `active_surface_id` | current shell surface for view actions |
| `previous_surface_id` | shell surface to restore when focus returns |
| `selection_ref` | preserved selection when view or inspect actions switch context |
| `sky_profile_ref` | stable sky profile identity |
| `time_of_day` | canonical world environment time field |
| `weather_regime_ref` | stable weather regime identity |
| `cloud_profile_ref` | stable cloud profile identity |
| `world_validation_verdict` | retained world validation result id |
| `emitter_class_ref` | stable audio emitter class identity |
| `zone_profile_ref` | stable zone/reverb profile identity |
| `ducking_policy_ref` | stable priority/ducking policy identity |
| `listener_profile_ref` | stable listener profile identity |

## Material-centric field additions
Add canonical fields:
- `material_response_family_ref`
- `material_light_response_ref`
- `material_visual_response_ref`
- `material_acoustic_response_ref`
- `material_cheap_runtime_rung`
- `material_route_closure_ref`
- `light_transmission_class`
- `shadow_response_class`
- `visual_emit_family`
- `acoustic_resonance_family`


## Dream-stack deep field additions
| Field id | Law |
|---|---|
| `geo_anchor_ref` | stable geodetic anchor identity |
| `region_frame_ref` | exact region-frame identity |
| `cell_frame_ref` | exact cell-frame identity |
| `precision_zone_code` | declares precision/radius contract |
| `far_phenomenon_track_ref` | retained summary carrier for distant observable events |
| `field_family_id` | canonical world-field family id |
| `field_scope_class` | cell, surface, object-local, or volume |
| `field_conflict_verdict` | canonical conflict resolution result |
| `topology_ref` | stable structural-graph identity |
| `support_group_ids` | support-group ids for collapse legality |
| `load_path_ref` | stable load-path bundle id |
| `deformation_class` | crater/gouge/trench/asphalt/mud class |
| `dirty_volume_ref` | exact mutated terrain volume scope |
| `removed_mass` | retained mass-removal value |
| `deposited_mass` | retained mass-deposit value |
| `fluid_inventory_ref` | stable fluid inventory identity |
| `fill_ratio` | canonical normalized fill value |
| `leak_geometry_ref` | stable leak/orifice geometry identity |
| `contamination_class` | canonical contamination family |
| `freeze_state` | exact fluid freeze posture |
| `front_ref` | stable climate-front identity |
| `front_lifecycle_state` | front/cell lifecycle stage |
| `arrival_window` | retained arrival horizon for far weather |
| `lunar_phase_code` | canonical lunar phase identity |
| `shared_intent_code` | exact squad-intent family id |
| `cover_validity_digest_ref` | retained cover-legality digest |
| `need_vector_ref` | stable needs bundle ref |
| `status_delta_ref` | retained reputation/status delta ref |
| `species_topology_class` | canonical body-topology class |
| `organ_zone_ref` | stable organ-zone identity |
| `survivability_verdict` | stable biological consequence verdict |
| `gore_legality_code` | canonical gore legality class |
| `representation_rung` | exact fur/hair/foliage representation rung |
| `coverage_mask_ref` | stable coverage/groom mask identity |
| `lighting_rung` | exact lighting fallback rung |
| `shadow_rung` | exact shadow fallback rung |
| `volumetric_rung` | exact volumetric fallback rung |
