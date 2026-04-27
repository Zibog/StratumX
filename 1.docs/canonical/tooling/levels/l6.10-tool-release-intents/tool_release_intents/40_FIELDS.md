# tool_release_intents internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| release_intent_row_id | ReleaseIntentRowId | required | canonical storage row identity | stable within the sidecar store |
| release_intent_id | ReleaseIntentId | required | foreign key to the published release intent | must resolve to a root-level row |
| release_manifest_ref | ReleaseManifestRef | optional | manifest assembled for the release intent | required once release planning begins |
| approval_gate_ref | ApprovalGateRef | optional | approval gate used before publish | required when publish mode is not dry-run |
| intent_resolution_state | ReleaseIntentResolutionState | required | latest resolution state for the release intent row | must use declared enum |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_release_intents`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
