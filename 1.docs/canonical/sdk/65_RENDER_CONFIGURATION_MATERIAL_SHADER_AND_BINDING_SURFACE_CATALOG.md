# Render Configuration Material Shader And Binding Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

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
