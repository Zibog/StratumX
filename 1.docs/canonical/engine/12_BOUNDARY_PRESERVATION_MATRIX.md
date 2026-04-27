# Boundary Preservation Matrix

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose

This document records which earlier boundaries are preserved, widened, repositioned, or removed in the master canon.

## Matrix

| Earlier Boundary | Status in Master Canon | Preservation Form |
|---|---|---|
| `engine_collision` | preserved | internal element `levels/l2-critical-simulation/kinetics/40_COLLISION.md` inside `engine_kinetics` |
| `engine_rigidbody` | preserved | internal element `levels/l2-critical-simulation/kinetics/41_RIGIDBODY.md` inside `engine_kinetics` |
| `engine_ballistics` | widened | split into `levels/l2-critical-simulation/kinetics/42_TRAJECTORY.md` and `levels/l2-critical-simulation/kinetics/43_IMPACT.md` inside `engine_kinetics` |
| `engine_fluids` | preserved | internal element `levels/l2-critical-simulation/field/40_FLUID_FIELD.md` inside `engine_field` |
| `engine_fire` | widened | preserved through `levels/l2-critical-simulation/field/41_THERMAL_FIELD.md` inside `engine_field` |
| `engine_terrain` | preserved | internal element `levels/l2-critical-simulation/field/42_TERRAIN_FIELD.md` inside `engine_field` |
| `engine_destruction` | widened | preserved through `levels/l2-critical-simulation/field/43_STRUCTURAL_FIELD.md` inside `engine_field` |
| `engine_weather` | widened | preserved through `levels/l2-critical-simulation/field/44_ATMOSPHERIC_FIELD.md` inside `engine_field` |
| `engine_navigation` | preserved | internal element `levels/l2-critical-simulation/agents/40_NAVIGATION.md` inside `engine_agents` |
| `engine_perception` | preserved | internal element `levels/l2-critical-simulation/agents/41_PERCEPTION.md` inside `engine_agents` |
| `engine_ai` | widened | preserved through `levels/l2-critical-simulation/agents/42_DECISION.md` inside `engine_agents` |
| `engine_social` | widened | preserved through `levels/l2-critical-simulation/agents/43_SOCIETY.md` inside `engine_agents` |
| `engine_schedule` | preserved | internal element `levels/l2-critical-simulation/agents/44_SCHEDULE.md` inside `engine_agents` |
| `engine_dialogue` | removed as standalone crate | absorbed into `engine_generation`; dialogue is one use-case, not the crate boundary |
| `engine_streaming_runtime` | widened | preserved through `engine_stream_control`, `engine_residency_control`, and `engine_transfer_control` inside `L1.5` |
| `engine_memory_runtime` | repositioned and narrowed | preserved through `engine_memory_control` inside `L1.5` |
| `engine_network` | widened | preserved through `engine_net_transport`, `engine_net_sync`, and `engine_net_latency` inside `L2.5` |
| `engine_render` | widened | preserved through `engine_imaging` as image synthesis family |
| `engine_audio` | widened | preserved through `engine_acoustics` as acoustic synthesis family |
| `engine_material` | repositioned | preserved as `L0.5` shared world property substrate |
| `engine_content` | repositioned | preserved as `L3.2` resource system with startup-mediated runtime-pack consumption rather than direct `L1.5` code dependency |
| `engine_inference` | repositioned | preserved as `L3.0` model system |
| `engine_startup` | repositioned | preserved as `L4` startup |

## Reading Rule

A widened boundary preserves the earlier capability class but places it under a larger and more general canonical crate boundary.

A repositioned boundary preserves the crate but changes its level placement in order to better reflect execution cost, ownership type, or substrate role.
