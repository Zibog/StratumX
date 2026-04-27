# Celestial Weather And Storm Debug Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Expose legal authoring and verification for celestial model, climate bands, storm fronts, precipitation, fire-weather-smoke coupling, visibility media, and recovery under pressure.
It is the phase-2 `Fire/Weather Lab` plus `Storm Track Preview` surface.

## Command inventory
- `btn.sky.author_celestial_model`
- `btn.storm.bind_climate_band`
- `btn.storm.inspect_band_state`
- `btn.storm.simulate_front_travel`
- `btn.storm.compare_triplet`
- `btn.storm.capture_evidence`
- `btn.storm.recover_baseline`
- `btn.storm.certify_pack`
- `btn.fire.inspect_ignition_lock`
- `btn.fire.simulate_coupled_burn`
- `btn.fire.compare_coupling`
- `btn.fire.capture_evidence`
- `btn.fire.recover_coupling_baseline`
- `btn.fire.certify_coupling_pack`

## Overlay families
- `overlay.sky.celestial_clock`
- `overlay.storm.front_track`
- `overlay.storm.precip_band`
- `overlay.storm.visibility_media`
- `overlay.fire.ignition_lock`
- `overlay.fire.thermal_band`
- `overlay.fire.smoke_advection`
- `overlay.fire.failure_board`

## Inspector fields
- `celestial_model_id`
- `climate_band_id`
- `storm_front_state`
- `visibility_band_state`
- `ignition_lock_reason`
- `thermal_front_state`
- `smoke_advection_state`
- `baseline_ref`
- `artifact_ref`

## Disabled reasons
- `disable.sky.author_blocked`
- `disable.storm.bind_blocked`
- `disable.storm.inspect_blocked`
- `disable.storm.capture_blocked`
- `disable.fire.inspect_blocked`
- `disable.fire.simulate_blocked`
- `disable.fire.capture_blocked`

## Exact brutal proof slices
1. wet grass does not ignite.
2. sustained heat dries the target and ignition resumes.
3. smoke advects with wind and stays comparable after recovery.
4. distant storm visibly approaches and changes local visibility pressure.

## Compare and capture law
This lab may use only:
- `compare.storm.front_track`
- `capture.storm.visibility_bundle`
- `compare.fire_weather.coupling`
- `capture.fire_weather.band_bundle`

Certification packs allowed from this surface:
- `pack.storm_long_range_visibility`
- `pack.fire_weather_smoke`

Any certify, capture, compare, or recover action must lower through the routes named in `editor/110`.

## Current posture
`document_gold / doc_closed_impl_open`
