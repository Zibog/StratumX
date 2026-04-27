# Libraries

## External Library Baseline

| Library | Role in `engine_imaging` |
|---|---|
| `wgpu` | GPU-facing image synthesis backend. |
| `bytemuck` | GPU-bound data packing. |
| `glam` | Spatial math for visibility and camera-space transforms. |
| `serde` | Serialization for imaging descriptors where required. |
| `smallvec` | Low-allocation extraction and task metadata. |
| `tracing` | Imaging diagnostics. |

## Fit

Each listed dependency serves the primary role of `engine_imaging` and stays aligned to its pressure axis.
