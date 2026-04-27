# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| outliner_view_id | OutlinerViewId | active outliner view identity | unique per outliner host |
| hierarchy_projection_ref | HierarchyProjectionRef | tree projection backing the outliner | must resolve through scene/world suites |
| row_state_set | OutlinerRowStateSet | expanded/selected/locked row state | explicit and bounded |
| filter_state | OutlinerFilterState | current filters and search text | must remain editor-local |
| context_action_set | OutlinerActionSet | legal row/context actions | must stay bounded and command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `outliner_system` without stealing truth from neighboring levels or lower packages.
