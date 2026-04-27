# Native File Dialog And Project Path Human Grade UX Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Make file/project UX feel like a real production tool, not a debug shell.

## Required shell behaviors
- native open/save dialogs where lawful for the platform;
- recent projects with stable icons, path previews, and corrupted-entry diagnostics;
- unsaved-change badges at project/world/layout/profile level;
- explicit “what will be saved” summary before destructive path changes;
- no silent path rewrite, mount remap, or asset-relocation magic.

## Required inspector/status fields
- `project_root_ref`
- `world_root_ref`
- `last_saved_at`
- `pending_save_scope`
- `source_control_posture`
- `path_legality_code`

## Current posture
`document_gold / operator_surface_closed / implementation_open`
