# engine_memory_control

## Stack Position

L1.5. Runtime Resource Services

## Primary Role

memory control.

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
| Allocator Classes | Frame, transient, pool, and fallback allocation classes. | `40_ALLOCATOR_CLASSES.md` |
| Lifetime Model | Frame, tick, cell, scene, and session lifetime rules. | `41_LIFETIME_MODEL.md` |
| Pressure Response | Trim, reclaim, and emergency pressure-response model. | `42_PRESSURE_RESPONSE.md` |
