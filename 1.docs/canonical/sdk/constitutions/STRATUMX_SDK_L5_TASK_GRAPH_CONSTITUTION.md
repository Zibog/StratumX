# STRATUMX_SDK_L5_TASK_GRAPH_CONSTITUTION

## Scope
This constitution governs how task-graph related facts may appear in `L5`.

## Binding laws
- `L5` may export prerequisites, readiness markers, and gating verdicts only;
- `L5` may not schedule, prioritize, or orchestrate upper workflows;
- task-graph meaning must be derivable from cursors/epochs/refs/verdicts, not hidden process state.

## Audit checks
- local packs never rename workflow ownership as bridge truth;
- readiness markers remain typed and externally explainable;
- upper tooling keeps orchestration authority.
