# Domain Family Data Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

Domain families are split by data responsibility, not by feature mood.
Each family must declare:
- authority-facing minimal truth;
- command classes touching that truth;
- snapshot classes;
- index classes;
- derived classes;
- artifact classes;
- preview classes;
- cache classes;
- diagnostics classes;
- degradation priorities.

## Canonical family registry
The authoritative family registry for this package lives in `36_CANONICAL_FAMILY_REGISTRY.md`.
Family completeness, family uniqueness, and composition-only legality must be evaluated against canonical family identifiers from that registry, not against physical folder ordinal prefixes.

## Required family coverage for the target editor
- **world/partition families**: cells, regions, world chunks, data-layer and streaming legality
- **scene families**: entity hierarchy, component presence, prefab instances, overrides, local children
- **content families**: asset identity, dependencies, reverse dependencies, import state, processor queues
- **cinematic families**: timelines, tracks, bindings, shot lists, preview scrubbing
- **observability families**: diagnostics, trace, performance, validation visibility
- **pack/release families**: build targets, manifests, cook closure, release bundles
- **assistant families**: evidence packs, proposal visibility, reversible apply/revert

## Family law
Families compose planes and sidecars; they do not replace them.
If a family starts storing its own hidden mutable authority, it is no longer a family and the canon is broken.
Physical folder mount labels may overlap numerically, but canonical family identifiers may not.
