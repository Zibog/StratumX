# Threading

This threading contract belongs specifically to the project orchestration tooling level.

## Threading law
- work around project_orchestration_record_id, scope_id follows the declared ownership model of this level
- publication for project orchestration remains immutable after publish
- cancellation or supersede for project orchestration must be explicit rather than inferred from missing rows

## Operational note
This file remains active and package-specific for `l7.0-project-orchestration` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
