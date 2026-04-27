# Field Contract Rules

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

Hot-path bridge structs should prefer:
- fixed-width ids;
- dense enum classes;
- explicit version, epoch, and scope fields;
- bounded arrays or small vectors;
- explicit replay and invalidation classes;
- opaque ref carriers instead of editor-shaped payload bodies.

Bridge field contracts should make these distinctions explicit:
- id vs ref vs handle vs artifact-ref;
- fact vs verdict;
- snapshot vs batch vs cursor;
- transport header vs cold diagnostics payload.

Forbidden in hot-path bridge structs:
- recursive payload graphs;
- nested unbounded maps;
- editor UI payloads;
- prefab/local-override payloads;
- data-layer trees;
- raw binary world dumps;
- nullable semantic ambiguity where tagged enums are possible.

## Required authoring-facing bridge fields
When an `L5` publication is intended for upper authoring consumers, it must expose enough metadata for `L6` to build legal projections:
- stable identity carrier;
- publication epoch or monotonic sequence;
- scope or domain class;
- compatibility and legality class when relevant;
- invalidation or replacement semantics;
- payload locality class when large bodies are externalized.
