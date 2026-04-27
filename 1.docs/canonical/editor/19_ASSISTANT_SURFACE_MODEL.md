# Assistant Surface Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Responsibilities
- assistant dock and invocation surfaces
- proposal presentation
- evidence visibility
- apply/revert controls
- task progress and result surfacing
- context-sensitive assistant handoff from active views and suites

## Laws
- the assistant surface is a host over `L6A` and `L7A`; it is not the assistant runtime or planner
- assistant UI owns no editor truth
- assistant apply paths must remain visible and reversible

## Built-in assistant law
The assistant is a first-class dock in the main shell.
It must support:
- open/close without layout breakage;
- context pickup from viewport, outliner, inspector, diagnostics, and stage strip;
- proposal preview;
- explicit apply;
- explicit revert through one rollback anchor chain.

## Extension-aware assistant law
The assistant may invoke extension-provided capabilities only when:
- the extension is mounted legally;
- the capability grant is visible;
- the proposal still lowers through canonical routes.

No assistant action may bypass canonical command ids or route families.
