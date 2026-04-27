# Activation And Invalidation

This contract belongs specifically to the l7.f3 simulation meta family and describes family-only coordination.


## Activation posture for `simulation_meta_family`
- the family may warm members together when simulation campaign composition and simulation workflow meta and related members share locality
- inactive members may not leave hidden live state behind

## Invalidation posture
- refreshes caused by changes in simulation campaign composition and simulation workflow meta, authority-facing minimal truth: simulation campaign refs only, snapshot classes: simulation-campaign snapshots must stay bounded and explicit

## Operational note
This file remains active and package-specific for `l7.f3-simulation-meta-family` / `41_ACTIVATION_AND_INVALIDATION.md`.

## Scope note
The authority, dependency, and audit meaning of 41 ACTIVATION AND INVALIDATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
