# Transaction to Runtime Consequence Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes how an authoring transaction becomes a runtime-visible consequence without collapsing authoring truth into runtime truth.

## Canonical route
`editor mutation ask -> tooling intent -> transaction/preview/runtime route -> sdk packet/control surfaces if lower truth is needed -> engine/runtime consequence -> sdk observations -> tooling projections/diagnostics -> editor surfaces`

## Consequence classes
A transaction may end in one of four legal consequence classes:
- `authoring_only` — changed authoring truth only;
- `preview_only` — speculative/live preview without durable publish;
- `runtime_consequence` — lower runtime/world truth changed or was inspected;
- `artifact_consequence` — build/import/package product changed.

## Main law
A route may not silently jump from `authoring_only` to `runtime_consequence`.
If a task crosses that boundary, the bridge and lower truth owners must be explicit.

## Current posture note
The uploaded code proves both sides of this distinction:
- `EditorAuthoringSession` proves authoring-only or preview-only style consequences;
- `VerticalSliceSession` proves a real runtime consequence for one narrow combat/material slice.
