# tool_panel_refs internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| panel_ref_row_id | PanelRefRowId | required | canonical storage row identity | stable within the sidecar store |
| panel_ref_id | PanelRefId | required | foreign key to the published panel binding | must resolve to a root-level row |
| layout_slot_ref | LayoutSlotRef | optional | layout slot used by the panel | present when the panel is docked into a known layout slot |
| attached_view_ref | ViewRefId | optional | view currently hosted by the panel | present when a hosted view exists |
| hydration_state | PanelHydrationState | required | whether the panel is cold, warm, or hydrated | must use declared enum |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_panel_refs`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
