# Lighting Atmosphere Camera And Post Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the public bridge for lighting, sky/atmosphere, camera, exposure, and post-fx publication.

## Packet families
### `packet.render.environment_state.v1`
Required fields:
- `sky_profile_ref`
- `weather_regime_ref`
- `cloud_profile_ref`
- `fog_media_profile_ref`
- `sun_moon_state_ref`

### `packet.render.camera_sensor_state.v1`
Required fields:
- `camera_ref`
- `sensor_profile_id`
- `exposure_mode`
- `exposure_value`
- `tonemap_profile_id`
- `post_chain_digest`

### `packet.render.environment_reason.v1`
Required fields:
- `reason_trace_ref`
- `dark_frame_reason_code`
- `blown_highlight_reason_code`
- `volumetric_fallback_code`
- `old_floor_substitute_code`

## Law
The bridge must let upper layers answer:
- why the frame is dark or blown out;
- which sky/weather bindings are active;
- which old-floor substitutions are active.
