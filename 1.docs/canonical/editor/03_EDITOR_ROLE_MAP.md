# Editor Role Map

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## `L8` role
Host the editor shell, own all product-local UI state, route user interaction, present panels and views, and translate user intent into legal lower-stack requests.

## `L9` role
Provide universal domain-specific authoring experiences above canonical lower-stack families without owning game-specific truth.

## `L10` role
Provide bootstrap, import/export, graph, automation, scripting, plugin, scaffold, and dependency services as **editor-owned product/service surfaces** over lower tooling runtimes.

## `L11` role
Provide collaboration, review, approval, playtest, reporting, onboarding, and production-facing surfaces.

## Explicit UI ownership
`editor/` owns:
- shell layout and workspace composition
- viewport navigation and overlay presentation
- selection and focus presentation state
- outliner/content/inspector view state
- command palette/search state
- package manager, timeline, validation, console, and runtime inspector panels
- plugin-contributed docks and widgets

Tooling does not own the above.

## Never roles
- no `editor/` layer owns lower-stack transaction authority
- no `editor/` layer owns raw engine runtime
- no `editor/` layer owns project/game-specific truth
- no `editor/` layer creates a hidden parallel world/asset/package truth
- no `editor/` layer bypasses `L6`, `L6A`, `L7`, or `L7A`

## Product/request law
- `L8` and `L9` are request-and-view surfaces
- `L10` is a service-and-batch surface
- `L11` is an operations-and-review surface
