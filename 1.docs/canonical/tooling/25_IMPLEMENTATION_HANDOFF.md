# Implementation Handoff

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines the implementation handoff for the tooling stack that powers the editor.

## Build target
Implement a layered authoring runtime where:
- `L6` owns all editor mutation authority and all deterministic lower-runtime queues;
- `L6A` mediates assistant proposals and apply/revert;
- `L7` compiles studio-scale campaigns and governance;
- `L7A` reasons and plans without directly mutating authority;
- the editor product UI remains entirely above tooling.

## Critical outcomes
- minimal `authority_core`;
- explicit command and transaction law;
- immutable snapshots plus rebuildable indices;
- deterministic artifacts for import, bake, cook, build, and release;
- visible validation, diagnostics, and queue state;
- bounded runtime attach and preview systems;
- degradable caches that never become shadow truth;
- workspace coordination without product-UI ownership creep.

## Recommended build order
1. `L6.0` through `L6.4` (`authority`, `command`, `transaction`, `snapshot`, `index`)
2. `L6.5` and `L6.6` (`derived`, `artifact`)
3. `L6.11` validation runtime
4. `L6.12` preview runtime
5. `L6.13` build runtime with asset-processor and bake queues
6. `L6.14` release runtime
7. `L6.10` workspace runtime as session-safe coordination only
8. sidecar ref/publication surfaces for shell and tools
9. `L6A` apply/revert flows
10. `L7` campaign and governance meta
11. `L7A` planning and canon reasoning

## Explicit non-goals
- no monolithic mega-runtime with hidden sub-stores;
- no plugin-owned authority islands;
- no assistant-owned mutation path;
- no release system that bypasses build/runtime manifests;
- no tooling-owned layout, dock, panel, or widget state.
