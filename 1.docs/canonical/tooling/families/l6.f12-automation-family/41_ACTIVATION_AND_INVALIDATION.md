# Activation And Invalidation

This contract belongs specifically to the l6.f12 automation family and describes family-only coordination.


## Activation posture for `automation_family`
- the family may warm members together when automation editor views, automation specs, and automation diagnostics and related members share locality
- inactive members may not leave hidden live state behind

## Invalidation posture
- refreshes caused by changes in automation editor views, automation specs, and automation diagnostics, authority-facing minimal truth: automation edit intents, snapshot classes: automation snapshots must stay bounded and explicit

## Operational note
This file remains active and package-specific for `l6.f12-automation-family` / `41_ACTIVATION_AND_INVALIDATION.md`.

## Scope note
The authority, dependency, and audit meaning of 41 ACTIVATION AND INVALIDATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
