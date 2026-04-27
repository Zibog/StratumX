# Communication

## Publication classes
- publishes opaque object handles used by ingress controls and egress batches

## Synchronization surfaces
- object handle publish surface
- object handle lookup surface

## Communication law
Communication in `engine_object_handles` is envelope- or ref-based only. No hidden callback, side cache, or editor-facing side channel may bypass these surfaces.

## Operational note
This file remains active and package-specific for `l5.11-engine-object-handles` / `30_COMMUNICATION.md`.

## Scope note
The authority, dependency, and audit meaning of 30 COMMUNICATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
