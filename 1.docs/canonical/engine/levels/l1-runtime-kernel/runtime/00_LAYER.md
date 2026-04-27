# engine_runtime

## Stack Position

L1. Runtime Kernel

## Primary Role

Execution constitution and global parallel ownership.

## Canonical Scope

Lifecycle, time, phase model, scheduler, execution law, ingress, egress, apply law, diagnostics, frame pacing, low-latency path law, queue ownership law, and profile model.

## Boundary Rationale

Execution ownership must be centralized so the engine keeps one phase law, one scheduling law, one apply law, and one low-latency law per world instance.

## Canonical Consumers

- `engine_runtime_headless`
- `engine_runtime_realtime`
- `engine_stream_control`
- `engine_memory_control`
- `engine_net_transport`
- `engine_net_sync`
- `engine_net_latency`
- `engine_startup`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_handle` — Stable references.
- `engine_ecs` — ECS substrate.
- `engine_world` — World truth boundary.
- `engine_world_region` — Region substrate used by scheduler and authority windows.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Lifecycle | Runtime start, stop, pause, resume, and state transitions. | `40_LIFECYCLE.md` |
| Phase Model | Canonical phase order and boundaries. | `41_PHASE_MODEL.md` |
| Scheduler | Partition-aware scheduling and legal execution ordering. | `42_SCHEDULER.md` |
| Apply Model | Collection and integration of staged changes. | `43_APPLY_MODEL.md` |
| Profile Model | Shared runtime profile model used by runtime profiles. | `44_PROFILE_MODEL.md` |
| Resource Coordination | Legal coordination model for runtime resource services. | `45_RESOURCE_COORDINATION.md` |
| Low-Latency Frame Path | Canonical low-latency law for interactive profiles. | `46_LOW_LATENCY_FRAME_PATH.md` |
| Queue Ownership | Canonical queue and ordered-publication law. | `47_QUEUE_OWNERSHIP.md` |
