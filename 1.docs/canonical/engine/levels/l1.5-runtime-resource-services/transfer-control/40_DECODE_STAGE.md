# Decode Stage

## Role

Runtime decode ownership.

## Data Model

Decode consumes runtime-ready products or prepared pages only.
Raw asset decode on the low-latency presentation path is illegal.
All numeric decode ceilings are frozen by `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Canonical Decode Matrix

| Product class | Owner | Runtime legality |
|---|---|---|
| GPU texture transcode-ready pages | `engine_transfer_control` | legal on transfer workers only |
| geometry binary pages | `engine_transfer_control` | legal on transfer workers only |
| material or shader metadata | `engine_startup` or cached background prep | startup-only or cached background prep |
| raw authored assets | `engine_content` | runtime-illegal |
| streaming audio compressed pages | `engine_acoustics` | legal only on acoustics-owned decode workers |

## Decode Ownership Rule

- decode CPU ceilings and compressed-byte ceilings are profile-bound and owned by transfer or acoustics services;
- decode may not cross apply boundary;
- runtime kernel, world truth, and imaging may not perform fallback raw decode.

## Illegal Patterns

- raw source decode in `engine_runtime`, `engine_world`, or `engine_imaging`;
- decode work crossing apply boundary;
- hidden fallback to frame-local raw decode because a warmup path was missing.
