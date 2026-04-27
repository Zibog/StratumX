# tool_view_refs internal fields

## Internal canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| view_ref_row_id | ViewRefRowId | required | canonical storage row identity | stable within the sidecar store |
| view_ref_id | ViewRefId | required | foreign key to the published view binding | must resolve to a root-level row |
| projection_manifest_ref | ProjectionManifestRef | optional | manifest describing the view projection internals | present when the view materializes structured projections |
| render_surface_ref | RenderSurfaceRef | optional | surface used to render the view | present when the view has a render target |
| hydration_state | ViewHydrationState | required | whether the view is cold, warm, or hydrated | must use declared enum |

## Internal sidecar law
This file freezes the internal canonical storage/state contract for `tool_view_refs`. It may include bookkeeping required to materialize the published rows, but it may not invent separate semantic truth.

## Relationship to the published projection
- every externally visible row must still resolve through the corresponding root-level `40_FIELDS.md` publication contract;
- nested-only fields exist only to support materialization, hydration, routing, or retention inside the sidecar.
