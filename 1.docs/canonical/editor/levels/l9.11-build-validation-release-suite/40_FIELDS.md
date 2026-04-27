# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| bvr_suite_session_id | BuildValidationReleaseSuiteSessionId | active suite session | unique per context |
| validation_graph_ref | ValidationGraphRef | current validation/dependency graph | typed and explicit |
| bake_queue_ref | BakeQueueRef | current Bake Service queue projection | must resolve through tooling build/runtime |
| release_readiness_ref | ReleaseReadinessRef | current release readiness projection | typed and explicit |
| suite_action_set | BVRActionSet | legal validate/bake/build/release actions | bounded and command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `build_validation_release_suite` without stealing truth from neighboring levels or lower packages.
