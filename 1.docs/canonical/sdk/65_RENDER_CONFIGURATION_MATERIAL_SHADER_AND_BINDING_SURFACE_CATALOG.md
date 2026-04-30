# Render Configuration Material Shader And Binding Surface Catalog

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Define the public bridge for render configuration, material/shader binding posture, and backend-legal variant publication.

## Packet families
### `packet.render.config_profile.v1`
Required fields:
- `render_profile_id`
- `backend_class`
- `feature_tier_code`
- `optional_accelerator_set`
- `material_policy_ref`

### `packet.render.shader_variant.v1`
Required fields:
- `shader_family_id`
- `source_language_id`
- `target_set`
- `variant_key`
- `fallback_variant_key`
- `compile_verdict_code`

### `packet.render.binding_scope.v1`
Required fields:
- `material_profile_ref`
- `resource_layout_id`
- `binding_group_digest`
- `missing_binding_code`
- `residency_hint_code`

## Law
- authoring source may remain HLSL-first while target sets vary by backend;
- variant publication must not hide fallback to a cheaper or alternate target;
- missing-binding posture must be machine-readable and operator-visible.

---

# V34 hybrid material packet closure

Stack version: `SX-CANON/1.0.28/STACK-v34`

SDK material packets must expose the hybrid model explicitly.

## Required packet rows

| Packet | Fields |
|---|---|
| `material.profile.v1` | material_id, family_id, parameter_schema_id, texture_bindings, response_profile_refs |
| `material.shader_variant_request.v1` | family_id, feature_bits, texture_channel_set, backend_target_set |
| `material.texture_channel_verdict.v1` | texture_id, channel, color_space, compression, mip_status, missing_reason |
| `material.preview_result.v1` | material_id, preview_surface, backend_class, frame_id, blockers |
| `material.graph_compile_result.v1` | graph_id, output_family_id, parameter_block, diagnostic_source_map, compatibility_verdict |

Graph packets are allowed only as authoring packets. Runtime packets consume compiled profile rows.
