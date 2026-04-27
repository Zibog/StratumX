# Editor Glossary

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

editor shell: product-owned workspace frame, menu, toolbar, status strip, dock host, and command hub.

viewport: a 3D or 2D render surface for world visualization. Viewports consume world truth via `L6.0 authority_core` projections and preview outputs via `L6.12 preview_runtime`.

outliner: hierarchy and world-browser projection over entity, region, layer, and runtime-loaded state.

inspector: product-owned details surface for staged edits, diffs, validation hints, and prefab/runtime comparison.

data layers: editor-visible and runtime-meaningful layering system over world content.

prefab: reusable authored object with lineage, override, nested, and variant posture.

asset processor: lower `L6.13 build_runtime` service responsible for watch, import, reimport, artifact build, quarantine, and remediation; surfaced through editor UI but not owned by it.

runtime inspector: editor surface that displays runtime projections and authoring-vs-runtime diffs through bounded lower-runtime bindings.

package manager: editor surface that presents package/dependency state and lowers legal package/build/release requests to tooling.

World Outliner / Scene Tree: exact surface label for the hierarchy panel that projects entities, folders, regions, layers, runtime-loaded posture, and selection state.

Details / Inspector: exact surface label for the property editor that stages edits, shows diffs, and exposes component/prefab/runtime posture.

Content Browser / FileSystem: exact surface label for the asset browser and FileSystem Dock style project-content view.

Command Palette / Search: exact surface label for the universal search and command dispatch entry point.

World Partition: exact large-world partitioning posture used for cell, region, load-mask, and streaming preview workflows.

One File Per Actor / External Actors: exact source-control-friendly storage posture for large-world authored objects alongside one-file-per-entity.

Level Instancing: exact authored-world reuse posture for chunk, region, and reusable world composition.

Nested Prefab: exact reusable authored-object pattern where prefab lineage can nest.

Prefab Variant: exact reusable authored-object pattern where variants inherit from a source prefab.

Prefab Editing Mode: exact isolated editing posture for prefab source editing.

Package model: exact package/dependency/module posture used for tool and content distribution.

Scene Tree: alias of World Outliner / Scene Tree.

FileSystem Dock: alias of Content Browser / FileSystem when used as a docked project-file browser.

EditorPlugin.add_dock: exact plugin-extension phrase for mounting editor docks through the plugin host.

Entity Inspector: exact component-oriented inspector posture used for entity/component authoring.

Entity/Prefab workflows: exact reusable authoring workflow family for entity and prefab operations.

Procedural Prefab workflows: exact authoring workflow family where procedural systems instantiate prefab-driven content.

Package Manager: exact panel and service label for package/dependency operations.

Build Status: exact field label for surfacing build-state visibility.

Prefab Apply/Revert: exact tool label for apply/revert authoring actions.

PIE Attach: exact runtime-bridge attach label for play-in-editor bindings.

Runtime Watch: exact runtime inspection/watch label for live property observation.

Cell Load/Unload: exact world-partition load-control label for cell operations.

Bake Service: exact service label for bake/cook operations surfaced in the editor.
