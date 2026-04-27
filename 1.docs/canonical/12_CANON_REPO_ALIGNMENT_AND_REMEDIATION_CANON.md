# Canon, Repo Alignment, and Remediation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the code-aware normalization plan for turning the current repository from split-brain posture into one honest editor product spine.
It is not a marketing note.
It is a hard alignment document for humans and neural implementation systems.

## Block 01 — one-canon law
The repository must converge on one active canonical tree.
The canonical tree shipped by this archive is the target truth.
Repository-local duplicates, stale aliases, or package spillover files are not equal law.

## Block 02 — one-host law
The product may have exactly one canonical editor host.
For this canon the canonical host is a Rust-native desktop editor host rooted under `6.apps/editor/`.
The React/Vite/Tauri package archived under `8.examples/_obsolete/stratumx_editor_ui_preview_pkg` is migration evidence and preview experimentation only.
It is not product truth.

## Block 03 — host evidence in current code
Current repository evidence shows two competing host paths:
- `6.apps/editor/stratumx_editor_app`
- `8.examples/_obsolete/stratumx_editor_ui_preview_pkg`

`6.apps/editor/stratumx_editor_app/DEPRECATED.md` currently claims the opposite host truth.
That file is repository posture evidence, not canonical law.
The repository must be normalized to the one-host decision in this archive.

## Block 04 — library baseline normalization
Because the canonical host is Rust-native, the editor-local external baseline remains centered on:
- `winit`
- `egui`
- `egui-winit`
- `egui-wgpu`
- core Rust serialization, tracing, and id utilities

`react`, `vite`, `typescript`, and `tauri` are not canonical product-host baseline libraries.
If they remain in the repo they are explicitly non-authoritative.

## Block 05 — package-boundary purity
The engine package must not contain editor product documents.
The editor package must not contain engine truth definitions.
The cleaned canonical tree removes editor spillover from `canonical/engine/`.
If the codebase still mirrors that contamination, the codebase is behind canon and must be normalized.

## Block 06 — one-world law
The opened world, the world shown in the viewport, and the world entered for play/simulate must resolve to the same world identity unless an explicit forked-runtime mode is declared.
No hidden test world, fake preview world, or detached runtime world may masquerade as the authoring world.

## Block 07 — boot, open, restore, fallback law
The editor must boot into exactly one of these legal outcomes:
1. restore last valid workspace + last valid world;
2. restore last valid workspace + startup world;
3. default workspace + startup world;
4. explicit failure posture.

Silent blank-shell startup is forbidden.

## Block 08 — honest viewport law
Current code evidence shows `8.examples/_obsolete/stratumx_editor_ui_preview_pkg/src/components/viewport/ViewportCanvas.tsx` behaves like a local 2D schematic canvas.
That is not canonical viewport truth.
The canonical viewport must display a lower-stack frame product, not a local decorative reconstruction.

## Block 09 — terrain delivery law
Terrain must travel through a dedicated legal thread:
`engine terrain truth -> sdk terrain observations -> tooling terrain projections -> editor viewport terrain presentation -> diagnostics -> terrain suite controls`.
Terrain may be visually bare.
Terrain may not be fake.

## Block 10 — sky/environment delivery law
Sky/environment must travel through a dedicated legal thread:
`engine environment truth -> sdk environment observations -> tooling environment projections -> editor viewport environment presentation -> diagnostics -> environment suite controls`.
Environment fidelity may degrade.
Environment truth may not silently disappear.

## Block 11 — one-frame convergence law
Terrain and sky/environment are separate lower-stack threads that converge into one authoritative viewport frame contract.
The editor may not merge them by ad hoc local state.
The authoritative frame classes are:
- `ViewportBootFrame`
- `ViewportLiveFrame`
- `ViewportDegradedFrame`
- `ViewportFailureProjection`
- `ViewportPlayAttachedFrame`
- `ViewportSimulateAttachedFrame`

## Block 12 — dto family freeze
Current repository evidence shows semantic DTO drift between Rust bridge code and TypeScript UI code.
The canonical response is not to “let adapters improvise”.
The canonical response is to freeze one semantic DTO family set in `editor/31_SHARED_TYPE_REGISTRY.md` and require every transport or host layer to mirror it exactly.

## Block 13 — id family freeze
Current repository evidence shows mixed `u32`, `number`, and `string` entity identifiers.
That is forbidden.
The canonical editor-facing identity is one opaque stable id family.
Frontend serialization may be string-based, but that string is only an opaque transport form of the canonical stable id.

## Block 14 — control-surface truth
Current repository evidence shows dead or inert chrome in menu bars, toolbars, and viewport controls.
Canonical controls must resolve to exactly one of:
- local presentation toggle;
- result-bearing lower command;
- explicit disabled state with reason.

No silent inert control survives in the active spine.

## Block 15 — diagnostics truth
Diagnostics are not optional chrome.
Diagnostics must explain:
- which host is active;
- which world is open;
- whether terrain is bound;
- whether environment is bound;
- whether a live frame exists;
- whether runtime entry is allowed;
- why any control is disabled;
- why the product is degraded.

## Block 16 — runtime entry law
Play and simulate are part of the first product loop.
They must enter the same world that the viewport is already showing.
The product must support:
- runtime attach;
- grounded walk;
- runtime diagnostics;
- return to authoring;
- preservation of world identity.

## Block 17 — suite prioritization law
The first product-result loop is not allowed to go wide before it goes honest.
The hot spine is exactly:
- editor shell
- world open / restore
- primary viewport
- outliner
- inspector
- content browser
- diagnostics rail
- terrain suite
- environment suite
- play/simulate entry

Everything else may stay cold.

## Block 18 — mock and preview evidence law
Current code evidence includes mock backends, local fake sky state, and preview-heavy UI chrome.
Mocks may exist as development evidence.
Mocks may not define canonical product behavior.
Any feature closed only against a mock backend is not closed.

## Block 19 — repo hygiene law
Current repository evidence includes committed dependency caches and build outputs under the preview package.
They are not product truth.
A gold repository state must exclude:
- `node_modules/`
- `dist/`
- stale preview outputs
- stale nested-workspace truth competing with the canonical host path

## Block 20 — remediation order
The mandatory remediation order is:
1. clean the active canonical tree;
2. choose one host and demote the other;
3. align library baseline with that host decision;
4. freeze ids and dto families;
5. close boot/open/restore/fallback;
6. close terrain path;
7. close environment path;
8. close one honest frame path;
9. close play/simulate/walk/return;
10. harden diagnostics and control surfaces;
11. only then widen chrome and next-wave systems.

## Acceptance note
This canon is satisfied only when the repository, host choice, ids, dtos, controls, and viewport frame all agree with the same stack truth.
Until then, the codebase may be development evidence, but it is not a gold product spine.
