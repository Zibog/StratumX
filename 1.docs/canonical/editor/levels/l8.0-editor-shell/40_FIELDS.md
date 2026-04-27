# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| shell_session_id | ShellSessionId | shell session identity | unique per open editor shell |
| layout_schema_id | LayoutSchemaId | active shell layout schema | must resolve through workspace schema |
| dock_host_set | DockHostSet | registered dock host regions | explicit and bounded |
| global_mode_state | GlobalModeState | authoring/play/simulate mode | finite enum only |
| console_visibility_state | ConsoleVisibilityState | visibility posture for console/status surfaces | finite enum only |

## Field law
The records above are the minimum editor-owned state needed to drive `editor_shell` without stealing truth from neighboring levels or lower packages.
