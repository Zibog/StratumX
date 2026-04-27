# Editor Control Surface and Command Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file freezes the global command grammar for editor-visible control surfaces.

## Global command law
Technology labs may not stop at generic verbs only.
Every technology surface must expose:
- one explicit button id present in `110_EXACT_BUTTON_TO_ROUTE_MANIFEST_CANON.md`;
- one domain-specific act verb;
- one domain-specific compare verb;
- one domain-specific capture verb;
- one evidence append verb;
- one drilldown verb when failure or degradation exists.

## Disabled reason families
- no legal project/workspace
- no legal target selected
- route unsupported by active build profile
- diagnostics source unavailable
- compare baseline missing
- placeholder or unresolved dependency blocks action
- active transaction forbids parallel mutation
- no legal recovery action is currently available

## Post-action focus law
After failure, focus must move to the most relevant diagnostics or drilldown surface declared by `110`.
After success, focus may move only to the next legal step declared by the button manifest and tooling `81`.
