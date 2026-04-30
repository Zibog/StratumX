# Artifact Evidence And Freeze Signal Registry Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
This registry freezes canonical artifact, evidence, and freeze signal ids used across compare, capture, certification, and release review.

| Signal class | Canonical ids |
|---|---|
| artifact classes | `artifact.trace.bundle`, `artifact.compare.digest`, `artifact.failed_run.bundle`, `artifact.recovery.bundle`, `artifact.cert.bundle`, `artifact.floor.bundle`, `artifact.material.coverage.bundle`, `artifact.material.branch.bundle`, `artifact.material.freeze.bundle` |
| evidence rows | `evidence.compare.triplet`, `evidence.freeze.blocker`, `evidence.cert.pass`, `evidence.cert.fail`, `evidence.recovery.rerun`, `evidence.material.coverage`, `evidence.material.branch_readiness`, `evidence.material.freeze_triplet` |
| freeze signals | `freeze.review_required`, `freeze.blocker_open`, `freeze.blocker_cleared`, `freeze.signoff_ready`, `freeze.denied`, `freeze.material.branch_incomplete`, `freeze.material.proof_missing` |

## Minimum retained fields
- `artifact_ref`
- `artifact_class`
- `owner_route_id`
- `truth_owner_id`
- `baseline_ref` when applicable
- `failed_run_ref` when applicable
- `focus_target_id`
- `first_failure_code` or success code

## Law
- evidence append before triplet closure is forbidden where triplet compare is mandatory;
- freeze review without one retained artifact bundle is forbidden;
- artifact refs may not be reused for semantically different artifact classes;
- material branch coverage used for release proof must retain `artifact.material.coverage.bundle` or `artifact.material.branch.bundle` rather than emitting shell-only status text;
- material freeze readiness is denied when `freeze.material.branch_incomplete` or `freeze.material.proof_missing` is active.
