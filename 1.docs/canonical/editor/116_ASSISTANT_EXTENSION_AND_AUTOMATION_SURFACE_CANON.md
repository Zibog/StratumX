# Assistant Extension And Automation Surface Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the built-in assistant dock, extension manager, capability-grant model, and apply/revert lane.

## Assistant law
The assistant is a permanent shell citizen.
It must support:
- open on current context;
- proposal preview;
- affected-route list;
- apply;
- revert;
- preserved evidence lineage.

## Creator-conveyor assistant law
The assistant must be able to operate as a five-step creator orchestrator without becoming shadow truth.
The canonical high-level loop is:
1. understand the requested game or slice;
2. emit a domain plan across world / terrain / structures / materials / audio / simulation;
3. propose concrete routed actions and assets;
4. apply only through lawful editor routes;
5. validate and summarize the result with rollback anchors.

This lets the editor feel like a modern creative system without introducing a “make game magically” fraud button.

## Extension law
Extensions are legal only through:
- installed bundle identity;
- explicit mount scope;
- explicit capability grants;
- disabled-by-default posture after install;
- editor-owned docking and lifecycle.

## Capability-grant law
An extension may request:
- shell surface mount;
- import/export helpers;
- diagnostics viewers;
- assistant-side helper skills.

It may not request:
- direct world truth mutation;
- direct engine truth mutation;
- hidden network or filesystem authority outside declared tooling surfaces.

## Apply/revert law
Assistant proposals and extension-assisted automations must still lower through:
- canonical button/command ids when applicable;
- canonical route families;
- canonical rollback anchors.

## Assistant presentation law
The assistant dock must show:
- current project and world identity;
- current stage and focused selection;
- proposed affected routes;
- files/assets/surfaces likely to change;
- success / partial / blocked state;
- last rollback anchor;
- validation recommendations after apply.

The dock may host chat, cards, checklists, and previews.
It may not hide what it touched.

## Required buttons
- `btn.extensions.open_extension_manager`
- `btn.extensions.install_extension_bundle`
- `btn.extensions.toggle_extension_mount`
- `btn.assistant.open_dock`
- `btn.assistant.apply_proposal`
- `btn.assistant.revert_last_apply`

## Prohibitions
- no “magic fix” button with no route lineage;
- no mounted extension with hidden grants;
- no apply path without one revert anchor;
- no assistant-owned shadow truth;
- no assistant output that bypasses stage-local validation and focus restoration.
