# Heavy Domain Capture Replay And Comparison Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Publish one surface catalog for replay and compare on dream-scene domains.

## Replay packet families

| Packet family | Required fields | Delivery posture |
|---|---|---|
| `packet.replay.destruction.v1` | `replay_ref`, `topology_ref`, `time_window`, `baseline_ref`, `compare_mode_id`, `reason_trace_ref` | retained artifact or explicit operator query |
| `packet.replay.weather.v1` | `replay_ref`, `front_or_cell_ref`, `time_window`, `baseline_ref`, `compare_mode_id`, `reason_trace_ref` | retained artifact or explicit operator query |
| `packet.replay.fire.v1` | `replay_ref`, `thermal_scope_ref`, `time_window`, `baseline_ref`, `compare_mode_id`, `reason_trace_ref` | retained artifact or explicit operator query |
| `packet.replay.hydrology.v1` | `replay_ref`, `container_or_source_ref`, `time_window`, `baseline_ref`, `compare_mode_id`, `reason_trace_ref` | retained artifact or explicit operator query |
| `packet.replay.tactics.v1` | `replay_ref`, `squad_ref`, `time_window`, `baseline_ref`, `compare_mode_id`, `reason_trace_ref` | retained artifact or explicit operator query |
| `packet.replay.wound.v1` | `replay_ref`, `body_ref`, `time_window`, `baseline_ref`, `compare_mode_id`, `reason_trace_ref` | retained artifact or explicit operator query |
| `packet.replay.ballistics.v1` | `replay_ref`, `projectile_chain_ref`, `time_window`, `baseline_ref`, `compare_mode_id`, `reason_trace_ref` | retained artifact or explicit operator query |

## Comparison output law
Comparison output must include:
- numeric tolerance posture;
- first divergence point;
- active degradation ladder;
- recovery suggestion id;
- missing-artifact posture if a required bundle was absent.

## Failure families
- `CMP_NUM_*`
- `CMP_TIM_*`
- `CMP_DEG_*`
- `CMP_TRC_*`
- `CMP_ART_*`

## Consumer law
Certification and freeze surfaces consume retained artifacts only.
Debug labs may request explicit replays but may not silently mount certification bundles as transient preview data.
