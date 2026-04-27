# Editor Shell Workspace Extension And Assistant Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define exact routing law for workspace stage changes, layout operations, viewport-shell mutations, extension lifecycle, and assistant apply/revert.

## Route clusters
### Shell/workspace routes
- `route.shell.open_stage_world.v1`
- `route.shell.open_stage_simulation.v1`
- `route.shell.open_stage_capture.v1`
- `route.shell.load_layout.v1`
- `route.shell.save_layout.v1`
- `route.shell.reset_layout.v1`
- `route.shell.detach_panel.v1`
- `route.shell.split_viewport_vertical.v1`
- `route.shell.split_viewport_horizontal.v1`

### Extension routes
- `route.shell.open_extension_manager.v1`
- `route.extension.install_bundle.v1`
- `route.extension.toggle_mount.v1`

### Assistant routes
- `route.shell.open_assistant_dock.v1`
- `route.assistant.apply_proposal.v1`
- `route.assistant.revert_last_apply.v1`

## Exact routing duties
Every route must declare:
- preconditions
- invalidation scope
- cache ownership
- focus on success
- focus on failure
- retained artifact policy
- rollback anchor policy if recoverable

## Special laws
### Layout routes
Layout routes may mutate only shell-local truth.

### Split viewport routes
Split routes must check:
- present legality
- backend legality
- secondary-view budget legality

### Extension routes
Install and mount are separate steps.
Mount is disabled by default after install.

### Assistant apply/revert routes
Apply creates one rollback anchor and one affected-route list.
Revert consumes one rollback anchor and republishes focus to affected surfaces.

## Prohibitions
- no layout route may mutate world truth;
- no extension route may grant undeclared capabilities;
- no assistant route may apply with no rollback anchor.
