# tool_inspection_views internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| inspection_row_id | InspectionRowId | required | canonical storage row identity | stable within the sidecar store |
| inspection_view_id | InspectionViewId | required | foreign key to the published inspection view | must resolve to a root-level row |
| field_group_manifest_ref | FieldGroupManifestRef | required | manifest describing grouped field layout for the view | must resolve through declared artifacts or derived products |
| staged_edit_session_ref | StagedEditSessionRef | optional | internal staged-edit state linked to the view | present only when staged edits exist |
| inspection_owner_panel_ref | PanelRef | required | panel that hosts the view | must resolve through declared panel refs |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_inspection_views`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
