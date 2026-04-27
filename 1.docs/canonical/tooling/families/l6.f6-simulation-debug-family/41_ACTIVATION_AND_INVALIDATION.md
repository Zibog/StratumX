# Activation And Invalidation

This contract belongs specifically to the l6.f6 simulation debug family and describes family-only coordination.


## Activation posture for `simulation_debug_family`
- the family may warm members together when simulation debug views, probes, timeline views, and diagnostic overlays and related members share locality
- inactive members may not leave hidden live state behind

## Invalidation posture
- refreshes caused by changes in simulation debug views, probes, timeline views, and diagnostic overlays, authority-facing minimal truth: no domain authority beyond debug-session refs, snapshot classes: simulation debug snapshots must stay bounded and explicit

## Operational note
This file remains active and package-specific for `l6.f6-simulation-debug-family` / `41_ACTIVATION_AND_INVALIDATION.md`.

## Scope note
The authority, dependency, and audit meaning of 41 ACTIVATION AND INVALIDATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
