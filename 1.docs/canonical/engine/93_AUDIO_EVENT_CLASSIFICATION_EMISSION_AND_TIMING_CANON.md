# Audio Event Classification Emission and Timing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own audio event kinds, timing windows, and emission legality.

## Truth objects
AudioEventState, AudioEmitterState, AudioTimingState

## Runtime phases
event classify -> emitter bind -> timing resolve -> publish audio event

## Diagnostics
event suppressed, emitter missing, timing invalid

## Exact editor entrypoints
- audio command rows in `editor/110` via `editor/95`

## Required publications
- `obs.audio.*` with event class, emitter truth, timing window, occlusion chain, and zone mix verdict;

## Phase-4 brutal proof slices
- `surface_dependent_footsteps_and_occlusion_mix`;
- `weapon_and_explosion_audio_publish_spatial_reason_chain`;

## Old-floor evidence obligations
- one retained old-floor bundle proving audio degrade or suppression stays typed;
- one compare digest proving baseline/current/recovered mix truth;

## Current posture
`document_gold / doc_closed_impl_open / runtime_family_closed_in_docs`
