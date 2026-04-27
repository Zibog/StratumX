# L2.5. Network Runtime Services

## Level Role

Runtime-owned online transport, sync, and latency-control services.

## Crates

| Crate | Primary Role |
|---|---|
| `engine_net_transport` | Gameplay network transport service. |
| `engine_net_sync` | Interest, snapshot, delta, and ack service. |
| `engine_net_latency` | Prediction, reconciliation, rewind, and latency-control service. |

## Upward Role

This level provides online service boundaries to higher consumers without taking world authority away from `engine_world` or execution ownership away from `engine_runtime`.

## Downward Dependence

This level depends on runtime law, lower substrate, and world/region context and does not bypass them.
