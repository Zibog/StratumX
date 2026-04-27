# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| dashboard_surface_id | DashboardSurfaceId | active dashboard surface identity | unique per host |
| traceability_view_ref | TraceabilityViewRef | current traceability projection | typed and explicit |
| milestone_summary_ref | MilestoneSummaryRef | current milestone or pipeline summary | typed and explicit |
| filter_state | DashboardFilterState | active dashboard filters | editor-local and explicit |
| dashboard_action_set | DashboardActionSet | legal drill-down/export actions | command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `production_dashboard_and_traceability` without stealing truth from neighboring levels or lower packages.
