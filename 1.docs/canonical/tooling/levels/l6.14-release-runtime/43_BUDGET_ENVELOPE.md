# Budget Envelope

This contract belongs specifically to the release runtime level and may not be reused verbatim by another tooling level.


## Budget posture for `release_runtime`
- correctness of `release_run_id`, `release_manifest_id` is non-degradable
- lower-priority outputs such as release manifests; publication status streams degrade before correctness
- any drop or defer decision must be visible through `{key}` publications, never hidden in an unnamed cache

## Operational note
This file remains active and package-specific for `l6.14-release-runtime` / `43_BUDGET_ENVELOPE.md`.

## Scope note
The authority, dependency, and audit meaning of 43 BUDGET ENVELOPE is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
