# Scope

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

The editor canon defines a universal creation platform rather than a project-specific game editor.
It exists to host a powerful authoring product above the lower tools stack and below project/game specialization.

## In scope
- shell, panels, workspaces, and product-facing interaction
- viewport, outliner, content browser, details/inspector, overlays, and command palette
- universal domain authoring suites such as world, scene, terrain, material/lookdev, destruction, simulation/AI, weather/environment, animation/cinematics, audio/voice, UI/HUD, quest/event/logic, and validation/release
- editor services such as import/export, graph tooling, automation, scripting, plugin host, templates, and dependency/package handling
- collaboration, review, approval, playtest, and production-facing surfaces
- product dataflow, activation, resource discipline, testing, and freeze rules

## Out of scope
- lower-stack transaction authority owned by `L6`
- lower-stack assistant runtime authority owned by `L6A`
- lower-stack orchestration authority owned by `L7`
- lower-stack planning authority owned by `L7A`
- project-specific lore, game taxonomy, game rules, quest schemas, factions, and concrete game runtime packages

## Boundary rule
The editor canon is universal through `L11`.
Project and game specialization begins only above this package.
