# StratumX Editor Controls

## Shell
- Top row: Menu Bar + Main Toolbar
- Left: Place Actors / Modes / Landscape
- Center: Level Viewport
- Right: World Outliner + Details
- Bottom: Content Drawer / Output Log / Validation / Runtime

## Landscape workflow
1. Switch to `Landscape` mode.
2. In `Manage`, choose resolution and world size.
3. Click `Create / Rebuild Landscape`.
4. In `Sculpt`, use:
   - `LMB` to raise terrain
   - `Shift + LMB` to lower terrain
   - `Flatten`, `Smooth`, `Noise` via the tool buttons
5. In `Paint`, choose `Layer 0..3` and paint with `LMB`.

## Play mode
- Click `Play` in the toolbar.
- Move with `W A S D`
- Move vertically with `Q / E`
- Hold `RMB` and drag to look around
- Click `Stop` to return to edit mode

## Logging
- Terminal transcript is written by `scripts/run-editor-with-trace.ps1`
- UI actions are written by the editor into `artifacts/reports/editor-ui-trace-*.log`
- Terrain state is written to `artifacts/editor-terrain-state.json`
- Project state is written to `artifacts/editor-project.json`
