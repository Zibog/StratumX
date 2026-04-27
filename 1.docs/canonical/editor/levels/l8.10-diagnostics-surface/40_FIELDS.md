# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| diagnostics_surface_id | DiagnosticsSurfaceId | active diagnostics surface identity | unique per host |
| issue_view_ref | IssueViewRef | current issue view backing the surface | must resolve through tooling diagnostics views |
| filter_state | DiagnosticsFilterState | active diagnostics filters | editor-local and explicit |
| selection_ref | DiagnosticSelectionRef | currently selected issue | bounded and publishable |
| detail_panel_state | DiagnosticsDetailPanelState | detail expansion and grouping posture | editor-local and explicit |

## Field law
The records above are the minimum editor-owned state needed to drive `diagnostics_surface` without stealing truth from neighboring levels or lower packages.
