# Photoreal Diagnostic Fallback And Certification Packet Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Expose the packets required to certify visual quality honestly.

## Packets

| Packet | Required fields | Primary consumers |
|---|---|---|
| `packet.photoreal_feature_fallback.v1` | `scene_class_id`, `hardware_profile_id`, `fallback_family`, `primary_verdict`, `first_downgrade_reason`, `artifact_ref` | editor `126`, tooling `90–91` |
| `packet.shadow_tier_report.v1` | `scene_class_id`, `shadow_tier`, `receiver_scope`, `first_blocked_feature`, `artifact_ref` | editor `126`, `135`, tooling `90–91` |
| `packet.transient_light_budget_report.v1` | `scene_class_id`, `hardware_profile_id`, `budget_state`, `green_threshold`, `red_threshold`, `artifact_ref` | editor `135`, tooling `90–91` |
| `packet.volumetric_media_tier_report.v1` | `scene_class_id`, `media_tier`, `first_downgrade_reason`, `artifact_ref` | editor `126`, `135`, tooling `90–91` |
| `packet.reflection_strategy_report.v1` | `scene_class_id`, `reflection_strategy`, `first_downgrade_reason`, `artifact_ref` | editor `126`, tooling `90–91` |
| `packet.visual_certification_scene_result.v1` | `scene_class_id`, `hardware_profile_id`, `primary_verdict`, `blocker_family`, `artifact_ref` | editor `124`, `126`, tooling `88`, `91` |

## Catalog law
A packet family in this catalog is mandatory for visual certification runs.
A green screenshot without these packets is not a gold result.
