# Boundary Preservation

This contract belongs specifically to the l7.f4 release meta family and describes family-only coordination.


## Must remain outside `release_meta_family` ownership
- member-internal mutable authority rows for release campaign composition, release governance, and release reporting, authority-facing minimal truth: release campaign refs only, snapshot classes: release-campaign snapshots, index classes: release-campaign indices
- unrelated domain truth not declared in this family
- editor widget/layout state

## Operational note
This file remains active and package-specific for `l7.f4-release-meta-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
