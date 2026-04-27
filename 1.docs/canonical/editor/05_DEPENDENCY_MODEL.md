# Editor Dependency Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines package-wide editor dependency law and legal dependencies to lower packages.

## Legal dependencies to tooling
The editor may consume:
- `L6.0 authority_core` for committed authoring truth refs and minimal mutable state projections
- `L6.1 command_envelopes` and `L6.2 transaction_ledger` for legal mutation lowering
- `L6.3 snapshot_plane`, `L6.4 index_plane`, `L6.5 derived_plane`, `L6.6 artifact_plane`, and `L6.7 stream_plane` for read projections
- `L6.9 budget_runtime` for lower-runtime pressure visibility
- `L6.10 workspace_runtime` for session-safe lower-runtime bindings and public refs only
- `L6.11 validation_runtime` for all validation scans and verdicts
- `L6.12 preview_runtime` for discardable previews and runtime attach previews
- `L6.13 build_runtime` for import, reimport, bake, cook, and build requests/results
- `L6.14 release_runtime` for package/release requests/results
- `L6A` assistant runtime surfaces for proposal/evidence/apply-revert UI
- `L7` compiled campaign/governance/reporting surfaces where batch work is needed
- `L7A` planning surfaces only through bounded assistant pathways

## Legal dependencies to sdk
The editor may consume universal SDK/platform primitives only for shell concerns such as:
- windowing
- input
- serialization helpers
- math/geometry primitives
- immutable handle/ref types exposed through `L5`

The editor must not use SDK access as a backdoor around tooling authority.

## Forbidden dependencies
The editor must not:
- write directly to `authority_core` without command/transaction law
- bypass tooling to reach engine or raw `L5` for world/entity/asset/package operations
- own shadow world/entity/asset/package truth
- let plugin, assistant, or collaboration code maintain private authority islands
- treat preview state as committed truth
- let tooling own product-local UI state that belongs in editor

## Dependency discipline
The product layer presents, stages, asks, filters, compares, diffs, explains, and composes UI.
The lower tooling layer commits, validates, previews, builds, packages, and reports.
