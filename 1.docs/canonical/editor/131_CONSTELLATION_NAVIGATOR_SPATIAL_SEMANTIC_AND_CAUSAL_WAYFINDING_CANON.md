# Constellation Navigator Spatial Semantic And Causal Wayfinding Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze one signature editor feature that makes the shell feel uniquely powerful: a hybrid navigator that fuses spatial minimap, semantic dependency awareness, and causal recovery guidance.

## Why this exists
Large simulation editors fail when users lose three things at once:
- where they are in space;
- what the selected thing belongs to;
- why the system is currently failing or behaving strangely.

The constellation navigator exists to solve all three in one humane surface.

## Canonical posture
The constellation navigator is a dockable or viewport-embedded surface that may appear as:
- compact corner radar in the viewport;
- expanded side panel;
- full-screen navigation mode for giant-world debugging.

## Mandatory layers
### Spatial layer
Shows:
- current region / chunk / subterranean volume;
- camera position and bookmarks;
- selected object, group, structure, or terrain patch;
- nearby authored zones such as weather cells, audio zones, or AI zones.

### Semantic layer
Shows:
- selected item type and owning domain;
- upstream dependencies;
- downstream dependents;
- stale bindings or missing links;
- quick-jump targets to the owning labs or inspectors.

### Causal layer
Shows:
- current failure or warning source when applicable;
- recent relevant incidents from the world timeline;
- “why did this happen” breadcrumb hints;
- recovery anchors and compare bookmarks.

## Interaction law
The navigator must support:
- click-to-focus in viewport;
- click-to-open owning lab;
- filter by domain;
- pulse recent incidents;
- show only broken/stale links;
- collapse to a low-noise corner mode.

## Relationship to existing systems
The navigator does not create new truth.
It visualizes and connects:
- shell focus truth;
- outliner and inspector selection state;
- timeline incidents from `130`;
- diagnostics and why-debug surfaces from `80`, `100`, and `105`;
- giant-world representation from `65` and `122`.

## Human-grade law
The navigator must feel elegant rather than tactical-sim ugly.
This means:
- clean lines and restrained motion;
- readable labels on dark backgrounds;
- no military-map clutter by default;
- progressive disclosure from compact to deep modes.

## Signature value
This is the editor’s distinctive premium feature.
It makes the viewport feel less like “camera plus gizmo” and more like a living world cockpit with memory, orientation, and reason.

## Prohibitions
- no duplicate truth stores;
- no hidden debug-only ids that the rest of the shell cannot explain;
- no giant-world map that is visually impressive but semantically useless;
- no causality hints without a lawful route into the owning debug or validation surface.
