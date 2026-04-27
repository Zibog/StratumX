# engine_residency_control

## Stack Position

L1.5. Runtime Resource Services

## Primary Role

residency control.

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
| Residency Sets | Resident and evictable set model. | `40_RESIDENCY_SETS.md` |
| Refcount and Budgets | Refcount and residency-budget model. | `41_REFCOUNT_AND_BUDGETS.md` |
| Pressure Signals | Pressure signal and reclaim visibility model. | `42_PRESSURE_SIGNALS.md` |
