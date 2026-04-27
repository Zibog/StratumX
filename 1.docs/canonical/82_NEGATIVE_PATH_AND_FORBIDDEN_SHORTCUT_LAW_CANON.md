# Negative Path And Forbidden Shortcut Law Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Legal chains are not enough.
This file freezes forbidden paths that would otherwise become editor-side or tooling-side loopholes.

## Forbidden paths
| forbidden path id | Illegal path | Why forbidden | Required emitted code |
|---|---|---|---|
| `forbid.editor_to_sdk_direct` | editor surface writes sdk packet without tooling route | destroys route ownership and retry/recovery law | `DISABLED_PACKET_FAMILY_UNBOUND` |
| `forbid.editor_to_engine_direct` | editor surface mutates engine truth directly | bypasses route, packet, evidence, focus law | `DISABLED_ROUTE_OWNER_UNAVAILABLE` |
| `forbid.sdk_without_focus_target` | terminal or retryable result without `focus_target_id` | editor cannot navigate legal next step | `DISABLED_FOCUS_TARGET_UNRESOLVED` |
| `forbid.result_without_artifact` | compare/capture/certify/recover terminal result without `artifact_ref` | evidence/freeze chain becomes unverifiable | `DISABLED_ARTIFACT_REF_MISSING` |
| `forbid.compare_without_baseline` | compare launched without baseline | triplet law violated | `DISABLED_BASELINE_MISSING` |
| `forbid.recovery_without_failed_run` | recovery launched without failed-run lineage | recovery legality unverifiable | `DISABLED_FAILED_RUN_MISSING` |
| `forbid.hidden_degrade` | route silently drops quality rung | destroys honest pressure and proof law | `DISABLED_DEGRADE_RUNG_EXHAUSTED` |
| `forbid.silent_normalization` | schema normalization drops required fields or rewrites codes silently | breaks chain conformance | `DISABLED_SCHEMA_REVISION_MISMATCH` |
| `forbid.unnamed_failure_family` | packet/result/artifact uses non-registry failure family | destroys recovery mapping | `DISABLED_ROUTE_OWNER_UNAVAILABLE` |
| `forbid.evidence_before_triplet` | evidence append before compare triplet closure | freeze review becomes fake | `DISABLED_RECOVERY_RUN_MISSING` |

## Enforcement law
Every conformance test, freeze review, and button manifest must treat these paths as explicit blockers, not as implementation notes.
