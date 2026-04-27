# Panel and View Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Mandatory anchored panels
- viewport host
- outliner / scene tree
- data layers / layers
- content browser
- details / inspector
- validation / diagnostics
- console
- assistant
- build / bake / release
- runtime inspector / play bridge
- package manager / dependency graph
- timeline / sequencer

## Concrete dock inventory
- Validation Dock
- Console Dock
- Package/Module Dock
- Runtime Inspector Dock
- AI Nav Bake Dock
- Material Authoring Dock
- Quest Graph Dock
- Dialogue Dock
- Animation State Dock
- VFX Graph Dock
- World Metrics Dock

## Common optional panels
- graph editor
- reference graph / reverse-reference graph
- compare / diff viewer
- review / annotation browser
- performance HUD / budget dashboard
- world metrics
- shot list and blend graph

## Mandatory hot anchors on default boot
The default level-space boot must make the following surfaces immediately reachable without layout recovery:
- primary viewport host
- outliner / scene tree
- content browser
- details / inspector
- diagnostics anchor
- toolbar/status strip

Runtime inspector, validation summary, and weather/environment tools may start warm rather than hot, but they must not require a separate editor mode or second shell.

## Panel law
Panels are hosts for projections, requests, staged edits, and job states.
Panels do not own authoritative domain state.

## Panel interaction rules
- outliner, viewport, inspector, and runtime inspector share focused-object routing
- content browser, inspector, validation, and package manager share asset focus routing
- data layers and world tools share region/cell focus routing
- build, bake, validation, and package panels surface queue state from lower runtimes, never private queues
- plugin docks may extend the shell, but they remain editor-owned UI and tooling-owned truth-free
- every dock is either editor-native or mounted through the plugin host with explicit mount scope

## Exact panel title labels
- World Outliner / Scene Tree
- Details / Inspector
- Content Browser / FileSystem
- Data Layers / Layers panel
- Timeline / Sequencer
- Command Palette / Search
- Package Manager
- Runtime Inspector Dock
- Validation Dock
- Package/Module Dock

## Worldspace-first anchor law
For the first product-result closure, the anchored panels must support one immediate worldspace loop:
- viewport shows the opened reference world;
- outliner confirms world contents;
- inspector confirms selected terrain/environment data;
- content browser exposes world and environment assets;
- diagnostics explains world-open, terrain/environment bindings, play/walk posture, and degradation posture.

No separate temporary viewport window or second editor surface is allowed to carry the real worldspace while the main shell remains decorative.

## Additional mandatory surfaces
- Workspace/Stage Strip
- Layout Selector
- Extension Manager
- Assistant Dock
- Capture/Review Rail
- Detached Secondary Viewport Host

## Windowing law
A detached panel or viewport remains part of the same editor session.
Detached windows may not create:
- a second project truth;
- a second world truth;
- a second assistant timeline;
- untracked focus islands.
