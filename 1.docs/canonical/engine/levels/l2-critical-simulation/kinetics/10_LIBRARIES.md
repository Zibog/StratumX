# Libraries

## External Library Baseline

| Library | Role in `engine_kinetics` |
|---|---|
| `rapier3d` | Baseline rigid body and contact solver substrate where justified. |
| `glam` | Spatial math for motion and impact. |
| `serde` | Serialization for family descriptors. |
| `bitflags` | Compact mode and response flags. |
| `smallvec` | Low-allocation local contact and solve metadata. |
| `tracing` | Family diagnostics. |

## Fit

Each listed dependency serves the primary role of `engine_kinetics` and stays aligned to its pressure axis.
