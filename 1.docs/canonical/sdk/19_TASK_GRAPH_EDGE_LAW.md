# 19 TASK GRAPH EDGE LAW

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Scope
This law constrains how `L5` participates in task graphs used by upper tooling without becoming a hidden scheduler.

## Allowed edge classes
`L5` may expose task-graph edges only as:
- cursor/epoch prerequisites;
- snapshot availability edges;
- artifact readiness edges;
- legality/compatibility gating edges.

## Forbidden edge classes
`L5` may not own:
- editor workflow ordering;
- package-install or plugin-activation ordering;
- bake/build/release orchestration decisions;
- assistant planning edges;
- UI focus or interaction routing edges.

## Edge semantics
Every edge exported by `L5` must answer one of these questions only:
- is the source publication available?
- is the source publication newer than the consumer cursor?
- is the subject legal/compatible enough to continue?
- is the referenced artifact ready to consume?

## Audit checks
- no edge name implies shell, suite, package, or planner ownership;
- all edges are derivable from typed facts, cursors, epochs, refs, or verdicts;
- `L5` remains an evidence-bearing prerequisite layer, not a workflow graph brain.
