# engine_stream_control

## Stack Position

L1.5. Runtime Resource Services

## Primary Role

stream control.

## Canonical Scope

Runtime resource-service ownership local to this crate.

## Boundary Rationale

This crate exists so its ownership class stays explicit and does not collapse into runtime, rendering, content, or simulation families.

## Canonical Consumers

- higher simulation families, service layers, and startup where justified by local contracts.

## Downward Dependencies

See `20_DEPENDENCIES.md`.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Stream Activation | Cell/chunk activation and deactivation model. | `40_STREAM_ACTIVATION.md` |
| Prefetch and Eviction | Predictive pull and ordered drop model. | `41_PREFETCH_AND_EVICTION.md` |
| IO Scheduling | Async read scheduling and cancellation model. | `42_IO_SCHEDULING.md` |
