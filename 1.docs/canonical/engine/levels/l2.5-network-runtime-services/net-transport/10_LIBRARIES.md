# Libraries

## External Library Baseline

| Library | Role in `engine_net_transport` |
|---|---|
| `quinn` | QUIC transport, streams, and datagram-capable connection surface. |
| `bytes` | Packet buffer ownership. |
| `bitflags` | Lane and packet flags. |
| `smallvec` | Small envelope batches. |
| `tracing` | Transport diagnostics. |

## Fit

Each listed dependency serves the primary role of `engine_net_transport` and stays aligned to its pressure axis.
