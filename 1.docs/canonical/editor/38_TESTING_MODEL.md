# Testing Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Scope
This document defines the editor canonical testing contour.
It has three layers:
- **documentation closure tests** for the canon package itself;
- **implementation readiness target tests** that the canon requires before downstream runtime/editor code may claim runtime readiness;
- **todo-list handoff classes** that make the canon directly usable as a production worklist.

## Documentation closure classes
- shell composition and boundary legality
- viewport/navigation/manipulator legality
- outliner/content/inspector contract completeness
- panel anchor completeness
- prefab/variant/override contract completeness
- data-layer/World Partition/chunk and Level Instancing posture completeness
- service and plugin legality
- runtime-bridge and playtest surface legality
- source-control/workspace/history/recovery closure
- hidden-state audit
- budget/activation closure

## Implementation readiness target classes
- outliner/content/inspector sync tests
- Prefab Apply/Revert and prefab override diff/apply/revert tests
- data-layer and World Partition legality tests
- asset processor / reimport / dependency invalidation tests
- validation runtime integration tests
- Bake Service / build / release closure tests
- runtime bridge, PIE Attach, Runtime Watch, and runtime inspector tests
- plugin registration and isolation tests
- Package Manager and dependency service tests
- source-control-friendly save/chunk tests

## Todo-list handoff requirement
Every implementation-facing class above must be precise enough to become a task in code, CI, or release readiness tracking without re-interpreting the canon.
Absence of executed runtime tests is not a documentation gap if the exact executable target classes, artifacts, and ownership boundaries are already frozen here.

## Canon gold rule
Documentation gold for `editor/` requires documentation closure tests and a fully specified implementation readiness target matrix.
Downstream runtime-editor readiness additionally requires executed implementation readiness target tests.
The canon may define downstream runtime readiness requirements without pretending they already executed, and those downstream runtime requirements do not block canon gold for the documentation package.

## Anti-template audit class
- pairwise diff review across all `L8`, `L9`, `L10`, and `L11` local packs
- surface ownership review for every panel, tool, service, and family
- request/result and invalidation specificity review for adjacent editor levels
