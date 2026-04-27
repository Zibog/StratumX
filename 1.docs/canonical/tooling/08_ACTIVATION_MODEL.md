# Activation Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

Activation is split into four canonical modes.

## Direct mode
Path: `user/tool -> L6 -> L5 -> engine`
Use for ordinary deterministic editor actions.
`L7` and `L7A` stay asleep.

## Assisted mode
Path: `user/tool -> L6A -> L6 -> L5 -> engine`
Use for local assistant help that does not require multi-step planning.
`L7A` may stay asleep.

## Deep-planning mode
Path: `user/tool -> L7A -> L6A -> L6 -> L5 -> engine`
Use only for complex multi-step planning, migration, optimization, or canon-constrained work.

## Orchestration mode
Path: `L7 -> L6 -> L5 -> engine`
Use for compiled campaigns, workflow graphs, governance-bound runs, or release orchestration.

Activation law:
- `L6` is the only always-hot editor owner
- `L6A` is warm and request-driven
- `L7` and `L7A` are cold and wake only for explicit planning or orchestration work
- passive reads never create hidden runtime activation
