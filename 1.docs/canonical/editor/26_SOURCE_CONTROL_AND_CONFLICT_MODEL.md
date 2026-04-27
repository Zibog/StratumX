# Editor Source Control and Conflict Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines source-control posture for the editor product.

## Core law
The editor may integrate with source control and conflict tooling, but it must not replace them.
The editor surfaces status, lock posture, diff posture, and recovery guidance.
External VCS tools remain authoritative for repository operations.

## One-file-per-entity / chunk posture
For large-world authoring, the editor must support a source-control-friendly storage posture where possible:
- one-file-per-entity for authored world objects where legal
- one-file-per-actor / external actors posture for large-world collaboration where legal
- world chunks or region manifests are stored independently
- entity, prefab, scene chunk, and layer-manifest ownership is separable for review and diff
- autosave and recovery outputs stay separate from source-controlled artifacts
- data-layer and partition edits remain traceable to concrete files or manifests
- World chunk save never implies hidden global scene rewrite

## Editor responsibilities
- show source-control status in content and world surfaces
- expose lock and ownership posture
- surface conflicted, stale, or diverged authored objects
- launch external diff/merge/conflict tools where supported
- warn when a runtime or editor mutation targets conflicted content

## Forbidden
- editor-owned merge engines presented as source-control authority
- hidden writeback to source-controlled files outside declared save/build/release flows
