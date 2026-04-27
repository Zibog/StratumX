# Lighting Shadow Probe and Transient Emission Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own direct/indirect lighting, shadow validity, probe use, and transient emission such as muzzle flash.

## Truth objects
LightState, ShadowState, ProbeState, TransientEmissionState

## Runtime phases
collect lights -> choose shadow/probe path -> apply transient emitters -> publish lighting result

## Diagnostics
shadow budget exceeded, probe stale, transient suppressed, light culled

## Exact editor entrypoints
- sky/atmosphere presentation command rows in `editor/110` via `editor/94`

## Required publications
- `obs.light.*` with transient emission, shadow validity, probe path, and first blocker code;

## Phase-4 brutal proof slices
- `tunnel_flash_lights_media_and_shadows_cohere`;
- `dynamic_shadow_validity_survives_event_light`;

## Old-floor evidence obligations
- one retained bundle proving transient emission and shadow validity on an old-floor rung;
- one denial sample proving culled or suppressed light publication;

## Current posture
`document_gold / doc_closed_impl_open / runtime_family_closed_in_docs`
