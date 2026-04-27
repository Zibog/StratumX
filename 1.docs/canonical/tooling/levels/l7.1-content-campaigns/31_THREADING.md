# Threading

This threading contract belongs specifically to the content campaigns tooling level.

## Threading law
- work around content_campaigns_record_id, scope_id follows the declared ownership model of this level
- publication for content campaigns remains immutable after publish
- cancellation or supersede for content campaigns must be explicit rather than inferred from missing rows

## Operational note
This file remains active and package-specific for `l7.1-content-campaigns` / `31_THREADING.md`.

## Scope note
The authority, dependency, and audit meaning of 31 THREADING is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
