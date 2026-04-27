# Photoreal Feature Fallback And Old Hardware Certification Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Provide the operator surface that makes visual downgrade and old-floor certification explicit instead of mystical.

## Mandatory controls
- choose scene class and hardware profile;
- inspect shadow, volumetric, reflection, and transient-light tiers;
- inspect first downgraded feature;
- launch capture and golden diff;
- review certification verdict and blocker family.

## Required fields
`scene_class_id`, `hardware_profile_id`, `shadow_tier`, `volumetric_tier`, `reflection_strategy`, `first_downgrade_reason`, `certification_artifact_ref`

## Disabled reasons
`PHR_DISABLED_NO_SCENE_CLASS`, `PHR_DISABLED_HARDWARE_PROFILE_UNKNOWN`, `PHR_DISABLED_CAPTURE_NOT_READY`
