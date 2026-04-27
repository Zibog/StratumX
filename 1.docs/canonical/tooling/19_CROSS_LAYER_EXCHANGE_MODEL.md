# Cross-Layer Exchange Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Principle
The editor product owns visible UI and staged user intent.
The tooling stack owns authority, queues, validation, previews, build, and release.
The bridge carries engine-facing facts and ingress.
No surface may short-circuit that path.

## Canonical exchange lanes
### 1. Authoring mutation lane
`editor -> L6 command_envelopes -> L6 transaction_ledger -> L5 ingress -> engine`
Used for:
- entity/component edits;
- hierarchy and transform edits;
- prefab apply/revert/unpack/variant creation;
- layer/data-layer assignments;
- package-legal authoring changes.

### 2. Projection lane
`engine/L5 -> L6 snapshot/index/derived/artifact/stream -> editor`
Used for:
- outliner projections;
- content/dependency listings;
- inspector details and diffs;
- validation and diagnostics summaries;
- runtime-inspector projections;
- package and build/release status surfaces.

### 3. Preview lane
`editor -> L6 preview_runtime -> derived/artifact/stream results -> editor`
Used for:
- material/lookdev previews;
- animation/cinematic scrub previews;
- property-staging previews;
- runtime bridge previews;
- validation-aware speculative transforms.

### 4. Background processing lane
`editor -> L6 validation_runtime / build_runtime / release_runtime -> artifact/stream results -> editor`
Used for:
- validation scans;
- import/reimport;
- dependency rebuilds;
- navmesh bake;
- lighting bake;
- HLOD bake;
- collision bake;
- gameplay database bake;
- spawn tables bake;
- localization bake;
- content builds and packaging;
- release closure and bundle materialization.

### 5. Assistant lane
`editor assistant surface -> L6A -> L7A/L7 as needed -> L6 command/preview/validation`
Used for:
- proposal generation;
- diff/explain/refactor flows;
- apply/revert under transaction law;
- evidence-backed assistance.

## Exchange law
Every upward-facing result must be one of:
- immutable projection;
- discardable preview;
- deterministic artifact;
- bounded stream event.

Every downward-facing mutation must be one of:
- legal command;
- compiled task bundle;
- compiled governance or release bundle.

Tooling may expose refs and queues to the editor.
Tooling may not capture the editor's layout, panel chrome, dock state, widget truth, or product-local workspace state.
