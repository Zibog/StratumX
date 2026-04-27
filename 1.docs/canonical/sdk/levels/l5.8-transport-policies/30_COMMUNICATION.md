# Communication

## Publication classes
- publishes immutable transport rules consumed by packet ingress and egress publication

## Synchronization surfaces
- transport policy registry publish surface
- transport policy lookup surface

## Communication law
Communication in `transport_policies` is envelope- or ref-based only. No hidden callback, side cache, or editor-facing side channel may bypass these surfaces.

## Operational note
This file remains active and package-specific for `l5.8-transport-policies` / `30_COMMUNICATION.md`.

## Scope note
The authority, dependency, and audit meaning of 30 COMMUNICATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
