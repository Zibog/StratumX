# Communication

## Publication classes
- publishes immutable compatibility profiles used by packets, controls, and transport policies

## Synchronization surfaces
- profile registry publish surface
- profile lookup surface

## Communication law
Communication in `compat_profiles` is envelope- or ref-based only. No hidden callback, side cache, or editor-facing side channel may bypass these surfaces.

## Operational note
This file remains active and package-specific for `l5.6-compat-profiles` / `30_COMMUNICATION.md`.

## Scope note
The authority, dependency, and audit meaning of 30 COMMUNICATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
