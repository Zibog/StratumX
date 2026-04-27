# Internal Dependencies

## Dependency Baseline

| Dependency | Why `engine_startup` depends on it |
|---|---|
| `engine_core` | Base contracts. |
| `engine_world` | World truth boundary. |
| `engine_runtime` | Runtime constitution. |
| `engine_runtime_headless` | Headless runtime profile. |
| `engine_runtime_realtime` | Realtime runtime profile. |
| `engine_stream_control` | Runtime stream service wiring. |
| `engine_residency_control` | Runtime residency service wiring. |
| `engine_memory_control` | Runtime memory service wiring. |
| `engine_transfer_control` | Runtime transfer service wiring. |
| `engine_kinetics` | Critical simulation family wiring. |
| `engine_field` | Critical simulation family wiring. |
| `engine_agents` | Critical simulation family wiring. |
| `engine_net_transport` | Network transport wiring. |
| `engine_net_sync` | Network sync wiring. |
| `engine_net_latency` | Network latency wiring. |
| `engine_inference` | Model system wiring. |
| `engine_generation` | Model system wiring. |
| `engine_imaging` | Synthesis system wiring. |
| `engine_acoustics` | Synthesis system wiring. |
| `engine_content` | Resource system wiring. |

## Downward Pattern

`engine_startup` depends downward only on crates that supply lower contracts, lower substrates, or the required world/execution/resource/network boundaries beneath its role.
