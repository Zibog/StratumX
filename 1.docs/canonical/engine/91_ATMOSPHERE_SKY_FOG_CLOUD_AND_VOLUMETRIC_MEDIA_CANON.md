# Atmosphere Sky Fog Cloud and Volumetric Media Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own atmospheric media truth and volumetric representation used by rendering.

## Truth objects
AtmosphereState, FogField, CloudField, VolumetricMediaState

## Runtime phases
sample atmosphere -> resolve cloud/fog tiers -> volumetric integrate -> publish media result

## Diagnostics
summary cloud tier, fog disabled by budget, volumetric skipped

## Exact editor entrypoints
- sky/atmosphere presentation command rows in `editor/110` via `editor/94`

## Required publications
- `obs.sky.*` with cloud/fog tier, volumetric rung, horizon visibility, and weather-coupled media state;

## Phase-4 brutal proof slices
- `far_storm_visibility_matches_weather_and_media_state`;
- `volumetric_rung_drop_keeps_visibility_truthful`;

## Old-floor evidence obligations
- one old-floor atmosphere bundle showing the actual volumetric rung;
- one compare digest proving horizon visibility stayed coupled to weather truth;

## Current posture
`document_gold / doc_closed_impl_open / runtime_family_closed_in_docs`
