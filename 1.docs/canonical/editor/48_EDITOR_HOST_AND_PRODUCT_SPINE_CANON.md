# Editor Host and Product Spine Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the minimum honest product spine for the editor and the normalization rules required to keep implementation aligned with canon.
A rich shell with a broken spine is worse than a smaller shell with legal truth.

## 01. Host decision
The canonical product host is a Rust-native desktop editor host under `6.apps/editor/`.
The current repository may still contain:
- a deprecated Rust prototype path; and/or
- a React/Vite/Tauri preview package.

Neither may override this host decision unless the canonical archive is explicitly revised.

## 02. One-host law
Exactly one editor host is canonical for product implementation.
Any second host, preview shell, deprecated shell, or parallel app path must be marked non-authoritative.
The repository may contain experiments.
The product may not.

## 03. Current repository posture
Current repository posture is red.
Major failure classes include:
- split host candidates;
- dto drift between UI and lower stack;
- mixed identity families;
- a fake or decorative viewport path in preview code;
- skeletal editor crates wider than the working spine;
- dead chrome and half-wired command surfaces;
- obsolete world-named feature canons in older snapshots;
- package-boundary contamination in canonical docs.

## 04. Canonical spine
The first canonical spine is exactly:
- one host
- editor shell
- world open / restore / fallback
- primary viewport
- outliner
- inspector
- content browser
- terrain suite
- sky/environment suite
- diagnostics rail
- play/simulate entry
- return to authoring

Anything outside this list may remain cold until the spine is honest.

## 05. Keep / freeze / demote posture
### Keep
- one host path
- lower-stack preview/runtime path
- type registry
- diagnostics path
- first-closure suites and surfaces

### Freeze
- dto families
- canonical ids
- command ids and labels
- host decision
- world-open result classes
- frame classes

### Demote from product truth
- preview package UI chrome
- deprecated host experiments
- local schematic viewport renderers
- frontend-invented dto semantics
- world-specific feature aliases

## 06. Chrome-before-spine prohibition
No implementation may prioritize wide chrome expansion over:
- host normalization
- dto normalization
- world-open closure
- terrain closure
- sky/environment closure
- runtime-entry closure

## 07. DTO law
Frontend or host technology does not define dto shape.
The type registry and canonical bridge families define dto shape.
If frontend code or mocks diverge, frontend code is wrong by default.

## 08. Identity law
Selection, viewport, outliner, inspector, diagnostics, and runtime attach must use the same canonical identity family.
Mixed `u32` and `string` local truths are forbidden.

## 09. Repo-hygiene law
The canonical product path must not be buried under committed build artifacts or dependency caches.
`node_modules`, `dist`, stale preview artifacts, or nested-workspace truth are not product truth.

## 10. Neural-implementation law
Neural implementation systems must prioritize:
- normalization rules
- host/spine law
- type registry
- engine-to-viewport frame canon
- control-surface canon
- diagnostics law

They must not infer product truth from stale preview code, duplicated docs, or deprecated hosts.

## 11. Acceptance note
This canon is satisfied only when the product spine is honest and the rest of the editor can widen on top of it without breaking one-host, one-world, one-frame truth.

## Product-spine expansion
The honest product spine now includes:
- stage strip;
- workspace layout save/load/reset;
- assistant dock;
- extension manager;
- one honest split/detached secondary viewport path.

These are shell-grade capabilities.
They widen the product spine without changing lower-stack truth ownership.
