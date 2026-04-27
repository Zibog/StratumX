# Editor Shell Workspace Windowing And Docking Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the shell, workspace, saved-layout, detached-window, and docking law for the editor.

## Workspace law
The editor owns one workspace shell over one project/world truth.
A workspace is:
- one stage emphasis;
- one layout topology;
- one focused surface chain;
- zero or more detached windows that still belong to the same session.

## Canonical shell topology
The default shell is a five-zone workstation:
- **top command spine** for project identity, save/play/sim/build, and shell-wide search;
- **left world conveyor rail** for sequential domain stages;
- **center stage** for viewport, graph, matrix, or compare surface;
- **right inspector** for exact selected-object or selected-rule truth;
- **bottom strip** for timeline, jobs, content drawer, diagnostics, notifications, and assistant tabs.

This topology is the default human-grade posture.
Alternative layouts may detach or rebalance zones, but the operator must always be able to return to this baseline in one action.

## Required workspace features
- saved named layouts
- default layout reset
- stage strip integration
- detachable docks
- detached secondary viewport host
- consistent focus restoration
- remembered bottom-strip tab posture
- remembered inspector section-collapse posture
- left-rail compact and expanded modes

## Docking law
Panels may dock, tab, stack, detach, and reattach.
They may not:
- invent truth;
- fork the assistant timeline;
- hide diagnostics lineage;
- lose rollback anchors when assistant apply/revert is active.

## Layout persistence law
The shell must retain:
- layout id;
- active stage;
- detached-window topology;
- focused surface;
- viewport topology;
- bottom-strip active tab;
- inspector expansion state;
- left-rail density mode.

Layout persistence is editor-local truth only.
It may not mutate runtime or world truth.

## Recovery law
Every layout operation must have:
- last good layout anchor;
- invalid-layout denial code;
- one-path reset to default workspace.

## Required buttons
- `btn.window.load_workspace_layout`
- `btn.window.save_workspace_layout`
- `btn.window.reset_workspace_layout`
- `btn.window.detach_panel`
- `btn.workspace.open_stage_world`
- `btn.workspace.open_stage_simulation`
- `btn.workspace.open_stage_capture`

## Shell readability law
The shell must be readable under a deep-space dark theme.
This means:
- contrast is carried by value, hierarchy, and spacing rather than pure white text;
- critical actions stay legible in compact mode;
- panels never collapse into undifferentiated black slabs;
- world identity, stage identity, and selected-object identity remain visible at all times.

## Prohibitions
- no floating-window chaos as default posture;
- no hidden panel state that cannot be recovered;
- no detached viewport that becomes the only honest world view;
- no assistant-only panel that cannot return focus to the main shell;
- no shell mode that removes the left-rail world conveyor without an obvious replacement surface.
