# World Scale Operator Lane And Distance Representation Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Give the operator one dedicated lab for geodesy, representation rings, far observability, and far-world continuity.

## Mandatory tabs
- coordinate frame
- region and cell anchors
- representation rings
- far observability
- save / restore continuity

## Mandatory overlays
- ring class overlay
- precision warning overlay
- distant observability overlay
- far-world retained-summary overlay

## Mandatory inspector fields
`world_frame_id`, `region_anchor_id`, `cell_anchor_id`, `ring_class`, `rebasing_state`, `observability_verdict`, `far_summary_id`

## Required actions
- move rebasing anchor;
- inspect ring thresholds;
- request far-summary compare;
- inspect what survives at near / mid / far / retained layers.

## Disabled reasons
`WRL_DISABLED_NO_WORLD`, `WRL_DISABLED_REBASING_LOCKED`, `WRL_DISABLED_FAR_SUMMARY_MISSING`
