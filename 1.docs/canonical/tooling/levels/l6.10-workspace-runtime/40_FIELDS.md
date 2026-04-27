# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| workspace_session_id | WorkspaceSessionId | public coordination session id | must match a tool session |
| published_selection_ref_set | SelectionRefSet | selection refs published from editor into tooling | refs only, no widget ownership |
| published_focus_ref_set | FocusRefSet | focus refs published from editor into tooling | refs only, no widget ownership |
| published_panel_ref_set | PanelRefSet | panel refs published from editor into tooling | panel identity only, not layout truth |
| published_view_ref_set | ViewRefSet | view refs published from editor into tooling | view identity only, not rendered widget state |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `workspace_runtime` without consulting a hidden mirror.
