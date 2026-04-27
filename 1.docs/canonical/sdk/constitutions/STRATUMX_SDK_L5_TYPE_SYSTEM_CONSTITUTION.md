# STRATUMX_SDK_L5_TYPE_SYSTEM_CONSTITUTION

## Scope
This constitution governs the type system posture of the bridge package.

## Binding laws
- types must remain explicit, stable, and semantically narrow;
- enum-bearing fields must use declared closed enums rather than free-text classes;
- ids, refs, handles, cursors, epochs, and verdict classes must not collapse into ambiguous primitive aliases.

## Audit checks
- local fields use canonical type classes;
- registries explain type purpose and boundaries;
- implementation handoff can derive code-level types from the canon without semantic guessing.
