# Technology Mutation Guard And Disabled Reason Registry Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
This registry freezes canonical mutation-guard and disabled-reason ids so lawful editor behavior is machine-readable.

## Mutation guards
- `guard.no_editor_direct_to_sdk`
- `guard.no_editor_direct_to_engine`
- `guard.no_capture_mutation`
- `guard.no_compare_without_baseline`
- `guard.no_hidden_degrade`
- `guard.no_freeze_without_evidence`
- `guard.no_terminal_result_without_focus`
- `guard.no_material_behavior_without_registry`
- `guard.no_world_save_with_unresolved_material_truth`
- `guard.no_debug_promotion_into_base_shell`

## Disabled reason families
| Disabled reason id | Meaning |
|---|---|
| `disabled.project_not_open` | no active project context |
| `disabled.route_owner_unresolved` | route exists but owner resolution failed |
| `disabled.schema_revision_mismatch` | packet or artifact revision mismatch |
| `disabled.baseline_missing` | baseline absent for compare/recovery |
| `disabled.failed_run_missing` | failed run absent for retry/recovery |
| `disabled.artifact_missing` | required artifact absent |
| `disabled.focus_target_missing` | lawful next focus target absent |
| `disabled.freeze_prereq_missing` | freeze review missing retained bundle |
| `disabled.recovery_not_legal` | recovery action forbidden by current state |
| `disabled.material_archetype_missing` | no archetype selected for the target scope |
| `disabled.surface_family_missing` | surface family unresolved |
| `disabled.response_profile_missing` | response profile unresolved |
| `disabled.terrain_layer_unresolved` | terrain layer weights do not resolve to a legal material truth |
| `disabled.biome_overlay_illegal` | biome overlay illegal for the target context |
| `disabled.chunk_rebuild_invalid` | dirty region or rebuild scope invalid |
| `disabled.world_validation_blocked` | world validation blocked by unresolved day-zero authoring truth |
| `disabled.debug_action_demoted` | advanced debug action is intentionally unavailable in the base shell |

## Law
- every disabled button or blocked route must map to one canonical disabled reason id;
- free-text-only disabled explanations are forbidden;
- a mutation guard violation must surface one denial code and one next legal recovery action.


## Base-shell and implementation-tail guards
- `guard.no_promoted_button_without_manifest_row`
- `guard.no_mutation_without_invalidation_declaration`
- `guard.no_mutation_without_shell_publication`
- `guard.no_surface_switch_without_context_restore_rule`
- `guard.no_world_environment_mutation_without_pending_save_posture`
- `guard.no_audio_world_binding_without_world_linkage_rule`

## Additional disabled reasons
| Disabled reason id | Meaning |
|---|---|
| `disabled.surface_unavailable` | requested shell surface is not available in current posture |
| `disabled.sky_binding_missing` | world has no legal sky binding for the requested action |
| `disabled.world_validation_prereq_missing` | world validation inputs are incomplete |
| `disabled.audio_world_scope_missing` | audio authoring action lacks a selected world source or zone |

## Material-centric guards
Promoted material actions must disable when:
- no archetype is bound for a material-owned target
- required response family coverage is incomplete
- light branch incompatibility exists
- acoustic remap is illegal for current state modifiers
- requested cheap-runtime rung violates validator floor
