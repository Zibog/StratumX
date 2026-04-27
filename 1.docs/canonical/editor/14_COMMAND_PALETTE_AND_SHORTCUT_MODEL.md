# Command Palette and Shortcut Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Responsibilities
- one searchable entry point for commands, tools, panels, assets, entities, validation rules, settings, and recent objects
- context-sensitive command ranking based on active suite, active selection, and active viewport mode
- explicit discoverability for automation, bake, validate, diff, PIE attach, package, prefab, terrain, sky, and runtime-entry commands

## Required search domains
- Assets
- Entities
- Components
- Commands
- Tools
- Settings
- Console actions
- Docs snippets
- Validation rules
- Recently opened objects
- recent sessions
- panels and views
- package actions
- terrain commands
- sky/environment commands
- runtime-entry commands

## Search and dispatch law
The command palette may rank, filter, explain, and dispatch legal requests.
It may not commit hidden mutations by search alone; all mutating actions still lower as legal requests.
Every command must resolve to a visible request, result, or routed panel action.

## Control-surface law
Every toolbar button, menu item, and shortcut must map to:
- one command identifier;
- one visible command label;
- one owner surface;
- one lower-path request class or a presentation-only toggle;
- one explicit disabled reason family.

The command palette therefore doubles as the searchable index of the control-surface canon.

## Exact command-search title
The canonical surface label is **Command Palette / Search**.
