# Photoreal Tunnel Flashlight And Shadow Proof Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the operator surface for the hardest local visual proof slice: a dark tunnel with transient lights, sound, smoke, and debris.

## Mandatory controls
- scene-class selector
- light tier inspector
- shadow tier inspector
- volumetric tier inspector
- transient light budget view
- audio continuity badge
- capture / compare / golden diff launcher

## Mandatory outputs
- one live viewport with honest runtime chain
- one fallback report pane
- one capture summary pane
- one blocker / recovery pane

## Required fields
`scene_class_id`, `hardware_profile_id`, `shadow_tier`, `volumetric_tier`, `transient_light_budget_state`, `audio_continuity_verdict`, `first_downgrade_reason`

## Disabled reasons
`TUN_DISABLED_NO_PROOF_POCKET`, `TUN_DISABLED_NO_CAPTURE_TARGET`, `TUN_DISABLED_SCENE_CLASS_MISMATCH`
