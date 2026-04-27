# Communication

## Publication classes
- publishes capability rows consumed by legality gates and ingress control validation

## Synchronization surfaces
- capability registry publish surface
- capability lookup surface

## Communication law
Communication in `compat_capabilities` is envelope- or ref-based only. No hidden callback, side cache, or editor-facing side channel may bypass these surfaces.

## Operational note
This file remains active and package-specific for `l5.5-compat-capabilities` / `30_COMMUNICATION.md`.

## Scope note
The authority, dependency, and audit meaning of 30 COMMUNICATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
