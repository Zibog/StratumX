# Editor Visual Composition Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Primary composition
- top frame with menu bar, main toolbar, play/debug toolbar, and status strip
- central viewport region with overlay shelves, gizmo ribbons, and runtime-bridge indicators
- left-side outliner / scene tree and data-layers region
- bottom or secondary content browser, validation, and console region
- right-side inspector/details, package manager, and runtime-inspector region
- optional timeline/sequencer, graph, reference graph, and build/release docks
- detached secondary windows for additional viewports or heavy suites

## Mandatory anchored layout
1. shell frame and active workspace
2. active viewport and overlays
3. persistent anchors: outliner, data layers, content browser, inspector
4. persistent production anchors: validation, console, package manager, runtime inspector
5. optional suite panels: timeline, graphs, world metrics, review, capture

## Default level-space boot composition
The canonical first-launch or restored-launch posture is a world-authoring composition, not an empty shell.
The default visible composition must include:
- top frame with active workspace/project/world identifiers
- one focused primary viewport showing the restored or startup world
- left-side outliner / scene tree
- bottom or secondary content browser plus diagnostics anchor
- right-side details / inspector
- visible toolbar/status indicators for play/simulate/build/runtime posture

If boot cannot restore or open a world, the same composition must remain visible with an explicit failure state in the viewport and diagnostics surfaces.

## Layout law
- all docked surfaces are layout-managed through editor-owned `L8.7`
- detached windows are secondary hosts, not separate truths
- every visual surface declares hot/warm/cold posture
- closed panels are cold and release heavy resources
- tooling does not own any of this composition

## Default reference-world worldspace composition
When the configured startup world is the current validation world, the default level-space boot composition must read as one worldspace workstation:
- primary lit perspective viewport centered on the active world;
- visible terrain/landscape silhouette and walkable surface massing;
- sky/atmosphere/light result belonging to the same world;
- outliner/world browser anchored left;
- content browser and diagnostics anchor reachable without layout repair;
- details/inspector anchored right.

The composition must not rely on a decorative sky backdrop to hide missing world-open or missing terrain data.
The first impression must already be "editing a world".

## Canonical morphology extension
The default composition must now read as:
- menu bar
- project/world identity strip
- stage strip
- command toolbar
- central viewport stack
- left navigator stack
- right inspector stack
- bottom diagnostics/content/capture rail

The visual hierarchy must make the viewport dominant and the diagnostics rail reachable without collapsing the workspace.

## Stage-strip law
The stage strip must remain visible in the main shell and switch the surrounding workspace emphasis without destroying the primary viewport.
Stage switching is a shell-layout operation over one project/world truth, not a context amnesia button.
