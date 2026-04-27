# Cross-Domain Budget Orchestra Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Declare one operator-visible arbitration law across CPU, GPU, RAM residency, and disk/streaming pressure.

## Pressure vectors
| Vector | Primary owners | Example source families | Primary operator signal |
|---|---|---|---|
| CPU simulation pressure | engine runtime phases | culling, animation solve, trace collection, certification replay | `pressure.cpu.*` |
| GPU frame pressure | engine presentation families | pass graph, lighting, post, volumetrics, VFX/UI compose | `pressure.gpu.*` |
| RAM residency pressure | engine + residency families | textures, geometry, transient buffers, capture caches | `pressure.ram.*` |
| disk / streaming pressure | engine streaming + tooling artifact lanes | pack reads, warmup, capture writes, certification bundles | `pressure.io.*` |

## Arbitration law
- no package below root may invent a fifth hidden resource vector;
- every degrade step must declare `vector`, `threshold_code`, `degrade_step`, and `expected recovery trigger`;
- the operator must see the same vector family across editor, tooling, and sdk packets.

## Degrade ladder classes
1. `observe_only`
2. `trim_optional_observation`
3. `reduce_frequency`
4. `reduce_precision`
5. `reduce_quality`
6. `fallback_profile`
7. `scenario_abort_with_artifact`

## Required overlay families
- `overlay.pressure.cpu.timeline`
- `overlay.pressure.gpu.pass_cost`
- `overlay.pressure.ram.residency_bucket`
- `overlay.pressure.io.queue_and_warmup`
