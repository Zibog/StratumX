# L4 Sync Surfaces

## Upstream/downstream bridge surfaces
- version registry publish surface
- version lookup surface

## Sync law
`compat_versions` may synchronize with L4 only through the listed surfaces. Any new surface requires an explicit update to this file and the package dependency model.

## Operational note
This file remains active and package-specific for `l5.4-compat-versions` / `41_L4_SYNC_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 41 L4 SYNC SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
