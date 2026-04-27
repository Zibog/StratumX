# Boundary Preservation

This contract belongs specifically to the l6.f10 audio family and describes family-only coordination.


## Must remain outside `audio_family` ownership
- member-internal mutable authority rows for audio authoring, buses, routing views, and audio manifests, authority-facing minimal truth: audio edit intents, snapshot classes: audio snapshots, index classes: audio lookup indices
- unrelated domain truth not declared in this family
- editor widget/layout state

## Operational note
This file remains active and package-specific for `l6.f10-audio-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
