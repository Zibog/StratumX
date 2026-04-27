# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| batch_run_id | BatchRunId | active batch run identity | unique per batch invocation |
| batch_recipe_ref | BatchRecipeRef | recipe or automation plan being executed | typed and explicit |
| target_scope | BatchTargetScope | scope of the batch run | explicit and bounded |
| progress_view_ref | BatchProgressViewRef | progress projection for the run | bounded and replaceable |
| batch_outcome | BatchOutcome | ready/failed/cancelled state | finite enum only |

## Field law
The records above are the minimum editor-owned state needed to drive `automation_and_batch_service` without stealing truth from neighboring levels or lower packages.
