# L1.5. Runtime Resource Services

## Level Role

Runtime-owned streaming, residency, memory, and transfer services.

## Crates

| Crate | Primary Role |
|---|---|
| `engine_stream_control` | Runtime streaming, activation, prefetch, and IO scheduling control. |
| `engine_residency_control` | Runtime CPU/GPU residency, refcount, and pressure visibility. |
| `engine_memory_control` | Runtime allocators, pools, lifetimes, and pressure response. |
| `engine_transfer_control` | Decode, staging, upload, readback, and fence-bound release control. |

## Upward Role

This level provides runtime resource services to all higher simulation, network, and service layers without taking execution ownership away from `engine_runtime`.

## Downward Dependence

This level depends on runtime law, lower substrate, and startup-mediated runtime-pack inputs prepared outside the level.
It does not take upward crate dependencies on resource systems and may not bypass startup-mediated product flow.
