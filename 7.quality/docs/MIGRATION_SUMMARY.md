# MIGRATION SUMMARY

This archive converts the legacy verification layout into a single crate-driven quality layer:

- `7.tests` -> `7.quality/suites`
- `scripts` -> `7.quality/tasks/stratumx_quality_tasks`
- `artifacts` -> `7.quality/data/baselines/imported_previous_run`

Key repository changes:
- root workspace members now point at `7.quality/*`
- broken `6.apps/*` workspace members were corrected to the current real directories
- engine test manifests were repointed to the current real `2.engine/*` crate paths
- sdk/tooling/editor/link suites now use `stratumx_test_support`
