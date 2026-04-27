# Threading

This threading contract belongs specifically to the preview runtime tooling level.

## Threading law
- work around preview_run_id, preview_request_id follows the declared ownership model of this level
- publication for preview runtime remains immutable after publish
- cancellation or supersede for preview runtime must be explicit rather than inferred from missing rows

## Operational note
This file remains active and package-specific for `l6.12-preview-runtime` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
