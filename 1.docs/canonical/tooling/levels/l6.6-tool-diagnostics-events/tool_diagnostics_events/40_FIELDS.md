# tool_diagnostics_events internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| diagnostics_event_row_id | DiagnosticsEventRowId | required | canonical storage row identity | stable within the sidecar store |
| diagnostics_event_id | DiagnosticsEventId | required | foreign key to the published event | must resolve to a root-level row |
| dedupe_key | DiagnosticsDedupeKey | optional | dedupe signature for burst suppression | required only when burst suppression applies |
| payload_manifest_ref | PayloadManifestRef | optional | manifest for expanded event payload | must resolve when present |
| ack_state | DiagnosticsAckState | required | acknowledgement posture for the event | must use declared enum |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_diagnostics_events`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
