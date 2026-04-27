# L4 Sync Surfaces

## Upstream/downstream bridge surfaces
- L4 ingress packet publish surface
- packet rejection surface
- decode-to-control split handoff
- decode-to-observation split handoff

## Sync law
`link_ingress_packets` may synchronize with L4 only through the listed surfaces. Any new surface requires an explicit update to this file and the package dependency model.

## Operational note
This file remains active and package-specific for `l5.0-link-ingress-packets` / `41_L4_SYNC_SURFACES.md`.

## Scope note
The authority, dependency, and audit meaning of 41 L4 SYNC SURFACES is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
