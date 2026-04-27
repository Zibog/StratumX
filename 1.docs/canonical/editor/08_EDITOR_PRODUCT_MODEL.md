# Editor Product Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

The editor product is a coherent Rust-native game-construction environment for assembling large worlds, reusable content, runtime previews, and release-ready outputs.
It is viewport-centric, panel-rich, suite-extensible, assistant-assisted, validation-aware, and production-capable.

## Core product pillars
- one shell, many workspaces
- viewport-first world interaction
- boot-to-viewport level-space presence
- universal tool contexts and overlays
- strong panel/view composition
- reusable prefab, nested prefab, and variant authoring
- large-world partition, data-layer, one-file-per-entity, one-file-per-actor, and external actors posture
- service-backed asset processor, validation, bake, build, and release flows
- runtime bridge, PIE attach, and runtime inspection
- plugin and package extensibility without authority leakage

## Mandatory product surfaces
- shell frame
- one or more viewports
- default level-space boot posture
- world outliner / scene tree
- content browser / filesystem dock
- details / inspector
- data layers / layers panel
- timeline / sequencer
- command palette and shortcut system
- diagnostics / validation / console surfaces
- assistant surface
- build / bake / release surface
- package manager / dependency graph surface
- runtime inspector / play-simulate bridge surface

## Mandatory product tool families
- transform tools
- placement tools
- world partition and region tools
- prefab and override tools
- validation, diff, and runtime-inspection tools
- timeline and camera tools

## Exact MVP editor baseline
- Outliner
- Inspector
- Content Browser
- Viewport
- Transform tools
- Prefab create/open/apply/revert
- Component add/remove/edit
- Asset import + reimport
- Validation dock
- Play in editor
- Timeline basic
- Layer system

## Exact gold editor baseline
- World partition cells
- Data layers with runtime meaning
- One-file-per-entity
- One-file-per-actor / external actors posture
- Nested prefabs + variants + override diff
- Background asset processor
- Inspector plugins
- Dock plugin API
- Region-based streaming preview
- Bake service
- Runtime live inspection
- Package manager
- Validation graph + dependency graph
- HLOD/nav/light/content build tooling

## Product rule
The product may be broad, but it must remain activation-bounded, authority-clean, and resource-aware.
The editor exists to assemble a game, not to become a second hidden engine.
It must launch into useful level-space presence rather than an empty shell whenever a legal startup world exists.

## Exact borrowed pattern names and labels
The canon intentionally borrows and freezes the following exact pattern names as implementation anchors and searchable labels:
- World Outliner / Scene Tree
- Details / Inspector
- Content Browser / FileSystem
- Command Palette / Search
- Package Manager
- World Partition
- One File Per Actor / External Actors
- Level Instancing
- Nested Prefab
- Prefab Variant
- Prefab Editing Mode
- Package model
- Scene Tree
- FileSystem Dock
- EditorPlugin.add_dock
- Entity Inspector
- Entity/Prefab workflows
- Procedural Prefab workflows

## Exact must-have key field labels
- Name
- Stable ID
- Source Prefab
- Override State
- Validation State
- Layer
- Data Layer
- Streaming Policy
- Mobility
- Tags
- Dependencies
- Build Status

## Exact must-have tool and service labels
- Prefab Apply/Revert
- PIE Attach
- Runtime Watch
- Cell Load/Unload
- Bake Service
- Package Manager

## First product-result requirement
The first emotionally-correct product posture is not "panels exist".
It is "the world is already there and I can step into it".
For first product closure, the editor product must be able to launch into one unified worldspace posture where:
- the startup reference world is legally opened;
- a real terrain/landscape surface is visible in the primary viewport;
- sky/atmosphere is visible through the same legal world binding;
- play/simulate can enter one legal walkable runtime posture on that same world;
- diagnostics can explain success, degradation, or failure.

The current validation world may be a Dark Valley development derivative.
That does not make the editor a Dark Valley editor.
The product target remains a generic engine/editor closure for any future world that obeys the same contracts.
