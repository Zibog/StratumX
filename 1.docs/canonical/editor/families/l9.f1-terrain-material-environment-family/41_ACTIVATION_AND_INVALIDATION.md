# Activation And Invalidation

This contract belongs specifically to the l9.f1 terrain material environment family and is not interchangeable with another editor family.


## Activation posture for `terrain_material_environment_family`
- members may warm together when terrain, lookdev, and environment suites and related surfaces share locality or tooling dependencies
- inactive family members may not leave hidden live state

## Invalidation posture
- shared indices, diagnostics, or previews tied to terrain, lookdev, and environment suites, terrain/material/environment projections, preview hooks, terrain/material/environment requests may trigger bounded family refreshes

## Operational note
This file remains active and package-specific for `l9.f1-terrain-material-environment-family` / `41_ACTIVATION_AND_INVALIDATION.md`.

## Scope note
The authority, dependency, and audit meaning of 41 ACTIVATION AND INVALIDATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
