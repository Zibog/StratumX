# Asset Streaming IO Pressure And Warmup Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define warmup sequences, streaming pressure, and asset I/O orchestration as runtime laws.

## Runtime truth objects
- `warmup_state`
- `io_queue_state`
- `streaming_pressure_state`

## Runtime phases
`prewarm -> queue -> read/decompress -> publish residency consequence`

## Diagnostics and evidence
- diagnostics must expose ids, reasons, queue state, and policy choices;
- degradation must be explicit and traceable;
- forbidden shortcuts: no editor-local or tooling-local state may impersonate runtime truth.

## Phase-6 obligations
- mixed-pack regression must expose whether warmup succeeded, was deferred, or was denied;
- region-scale validation must expose I/O pressure posture, not only final residency verdict;
- freeze review remains blocked when a freeze-relevant product relay depends on an undocumented warmup shortcut.

## Current posture
- document posture: `document_gold`;
- implementation posture: `doc_closed_impl_open` unless proof says otherwise.
