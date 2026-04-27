# STRATUMX_L6A_ASSISTANT_RUNTIME_CONSTITUTION

## Scope
This constitution governs the assistant runtime layer `L6A`.

## Binding laws
- `L6A` owns assistant sessions, proposals, lowering, safety gates, apply/revert runtime, and model-request runtime;
- `L6A` consumes facts from lower layers but does not rewrite their ownership;
- human approval and safety posture remain explicit in assistant workflows.

## Audit checks
- session/evidence/proposal/safety/apply-revert layers stay distinct;
- assistant runtime remains bounded and reversible;
- no planner or editor product state is silently absorbed into `L6A`.
