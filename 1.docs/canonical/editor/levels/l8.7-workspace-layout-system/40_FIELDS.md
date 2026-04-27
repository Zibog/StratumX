# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| workspace_layout_id | WorkspaceLayoutId | active workspace layout identity | unique per workspace session |
| dock_tree_ref | DockTreeRef | dock/tab tree for the workspace | must remain editor-owned |
| saved_layout_slot | SavedLayoutSlot | named slot for persisted layouts | bounded and explicit |
| migration_state | LayoutMigrationState | schema migration posture for layouts | finite enum only |
| restore_point_ref | RestorePointRef | restore point for workspace recovery | must resolve through autosave/recovery rules |

## Field law
The records above are the minimum editor-owned state needed to drive `workspace_layout_system` without stealing truth from neighboring levels or lower packages.
