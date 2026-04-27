# tool_content_intents internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| content_intent_row_id | ContentIntentRowId | required | canonical storage row identity | stable within the sidecar store |
| content_intent_id | ContentIntentId | required | foreign key to the published content intent | must resolve to a root-level row |
| authoring_template_ref | TemplateRef | optional | template or preset used to interpret the intent | must resolve when present |
| batch_group_ref | BatchGroupRef | optional | batch grouping for combined content operations | must be absent for singleton execution |
| last_resolution_state | ContentIntentResolutionState | required | latest resolution state for the intent row | must use declared enum |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_content_intents`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
