# tool_assistant_intents internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| assistant_intent_row_id | AssistantIntentRowId | required | canonical storage row identity | stable within the sidecar store |
| assistant_intent_id | AssistantIntentId | required | foreign key to the published assistant intent | must resolve to a root-level row |
| lowering_plan_ref | LoweringPlanRef | optional | lowering plan prepared for the intent | required once the intent enters lowering/apply flow |
| safety_gate_ref | SafetyGateRef | optional | safety gate linked to the intent | required when approval state requires gated review |
| intent_resolution_state | AssistantIntentResolutionState | required | latest resolution state for the assistant intent row | must use declared enum |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_assistant_intents`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
