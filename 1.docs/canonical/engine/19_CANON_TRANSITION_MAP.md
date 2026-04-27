# Canon Transition Map

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose

This document freezes the transition from the earlier split lower/upper canonical packages into the current unified master canon.

It exists to prevent naming drift, boundary drift, and undocumented reinterpretation of crate roles.

## Rules

- The master canon is authoritative.
- Earlier package names remain valid only as historical references unless explicitly preserved here.
- A changed boundary is canonical only when its preservation or widening is documented here.

## Level Transitions

| Earlier Canon | Master Canon | Transition Type | Canonical Meaning |
|---|---|---|---|
| `L2. Physical Domains` | `L0.5 + L2` | repartition | shared physical properties moved to `L0.5`; critical physical compute remains in `L2` |
| `L2. Behavioral Domains` | `L2. Critical Simulation Families` | merge and regroup | behavioral compute is preserved inside `engine_agents` |
| `L2. Inference and Dialogue Domains` | `L3.0. Model Systems` | regroup and widen | model-facing tasks move above critical simulation |
| `L2. Presentation and Resource Domains` | `L3.1 + L3.2` | regroup and split | synthesis and resource processing are separated |
| `engine_streaming_runtime` | `L1.5. Runtime Resource Services` | split and harden | streaming/residency/transfer responsibilities are split into explicit runtime service crates |
| `engine_memory_runtime` | `L1.5. Runtime Resource Services` | narrow and harden | runtime memory ownership is isolated into `engine_memory_control` |
| `engine_network` | `L2.5. Network Runtime Services` | split and harden | transport, sync, and latency control are separated |
| `L3. Startup` | `L4. Startup` | level shift | startup remains the highest launch layer |

## Crate Boundary Transitions

| Earlier Boundary | Master Boundary | Transition Type | Canonical Note |
|---|---|---|---|
| `engine_material` | `engine_material` | repositioned | moved from regular domain position to shared world property substrate at `L0.5` |
| `engine_collision` + `engine_rigidbody` + `engine_ballistics` | `engine_kinetics` | merged and renamed | preserved as one local dynamics family with collision, rigidbody, trajectory, and impact internal boundaries |
| `engine_fluids` + `engine_fire` + `engine_terrain` + `engine_destruction` + `engine_weather` | `engine_field` | merged and renamed | preserved as one distributed field family with fluid, thermal, terrain, structural, and atmospheric internal boundaries |
| `engine_navigation` + `engine_perception` + `engine_ai` + `engine_social` + `engine_schedule` | `engine_agents` | merged and renamed | preserved as one agent and society family with explicit internal boundaries |
| `engine_streaming_runtime` | `engine_stream_control` + `engine_residency_control` + `engine_transfer_control` | widened replacement | runtime streaming is preserved as three explicit service boundaries |
| `engine_memory_runtime` | `engine_memory_control` | renamed replacement | runtime memory ownership is preserved as one explicit service boundary |
| `engine_network` | `engine_net_transport` + `engine_net_sync` + `engine_net_latency` | widened replacement | multiplayer runtime concerns are preserved as three explicit service boundaries |
| `engine_inference` | `engine_inference` | repositioned | moved from L2 to `L3.0` as a model system |
| `engine_dialogue` | `engine_generation` | widened replacement | dialogue is not preserved as a standalone crate; generative output work is widened into `engine_generation` |
| `engine_render` | `engine_imaging` | widened replacement | render-facing concerns are preserved and widened into image synthesis |
| `engine_audio` | `engine_acoustics` | widened replacement | audio-facing concerns are preserved and widened into acoustic synthesis |
| `engine_content` | `engine_content` | repositioned | moved from L2 presentation/resource grouping to `L3.2. Resource Systems`; runtime-pack relation is startup-mediated rather than a direct `L1.5 -> L3.2` crate dependency |
| `engine_startup` | `engine_startup` | level shift | moved from earlier L3 to canonical `L4` |

## Preservation Rule

When an earlier standalone crate is merged or split into family/service crates, its role must remain visible through:
- explicit internal element documents;
- explicit dependency baselines;
- explicit boundary preservation notes.
