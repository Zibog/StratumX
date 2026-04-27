# STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW

## 1. Purpose

This document freezes visibility hierarchy, extraction law, transfer law, visibility staleness, and GPU upload/residency split for `engine_imaging` and `engine_transfer_control`.

## 2. Minimum Visibility Hierarchy

Canonical visibility must contain all of the following stages in order:
1. region or cell visibility selection;
2. instance-cluster visibility selection;
3. occlusion stage boundary before material submission;
4. submission-bucket formation for legal material or surface classes.

A flatter path is illegal on canonical realtime profiles.

## 3. Extraction Law

- extraction consumes read-only authoritative or staged view data only;
- extraction snapshots are frame-local and may not mutate world truth;
- extraction must obey scene extraction ceilings frozen by `STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`.

## 4. Transfer Law

- transfer ownership remains in `engine_transfer_control`;
- `engine_imaging` may request residency and transfer products but may not decode raw source assets or own upload lifetime directly;
- same-frame visibility of newly requested resources is legal only when residency, staging, and fence law all say the resource is publishable.

## 5. Visibility Staleness Law

- realtime visibility results may be at most 1 frame stale;
- staler visibility is legal only in explicit diagnostics or benchmark modes;
- visibility history may not silently accumulate as a hidden queue.

## 6. GPU Upload and Residency Split

On realtime profiles the canonical GPU-side split is:
- texture uploads: <= 16 MiB p95 and <= 32 MiB p99 per frame;
- geometry uploads: <= 8 MiB p95 and <= 16 MiB p99 per frame;
- transient frame resources: inside the transient VRAM class frozen by `STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`.

## 7. Optional Accelerator Law

- occlusion queries are legal optional accelerators;
- pipeline cache usage is legal optional acceleration only and may not become a required ownership boundary;
- mesh-pipeline or task-shader paths are legal optional accelerators only;
- all optional accelerators must preserve the canonical visibility hierarchy and the canonical low-latency path.

## 8. Illegal Patterns

- visibility implemented as one monolithic post-extraction list without region/cell and cluster stages;
- raw runtime decode inside imaging;
- hidden readback in low-latency presentation path;
- material permutation growth beyond the ceilings frozen by `STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`.
