# Workspace Layout and Savefile Identity Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze the durable shape of the project workspace so save/reopen, diagnostics, build, and freeze all point to the same project identity.

## Canonical workspace skeleton
- `/project/manifest/`
- `/project/content/source/`
- `/project/content/canonical/`
- `/project/world/`
- `/project/gameplay/`
- `/project/build/`
- `/project/evidence/`
- `/project/cache/local/`
- `/project/cache/reproducible/`

## Save identity contract
| Field | Meaning | Rule |
|---|---|---|
| project_id | stable project identity | never recycled |
| workspace_id | durable workspace identity | must survive reopen |
| savefile_id | concrete persistence record | new on each committed save |
| save_generation | monotonic save counter | must increase |
| schema_revision | documented save schema revision | must be explicit |
| content_snapshot_ref | canonical content snapshot used by the save | must be resolvable |
| world_snapshot_ref | world snapshot used by the save | must be resolvable |
| gameplay_snapshot_ref | gameplay snapshot used by the save | must be resolvable |
| diagnostic_seed_ref | optional replay/diagnostic anchor | must not masquerade as truth |

## Save-open rules
- local autosaves are never certification artifacts;
- reopen may migrate, but only with a recorded migration verdict;
- a savefile is invalid for build if any referenced canonical content is unresolved;
- save identity must roundtrip through editor, tooling, and sdk without renaming.

## Failure codes
- `SAVE_SCHEMA_TOO_OLD`
- `SAVE_SCHEMA_TOO_NEW`
- `SAVE_REF_BROKEN`
- `WORKSPACE_DRIFT_DETECTED`
- `MIGRATION_REQUIRED`
- `MIGRATION_FAILED`
