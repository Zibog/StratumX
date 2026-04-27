# STRATUMX_SDK_L5_SHARED_TYPE_REGISTRY_CONSTITUTION

## Scope
This constitution governs the shared type registry used by `L5`.

## Binding laws
- shared types must be canonical, stable, and narrow;
- registry entries must distinguish ids, refs, handles, cursors, epochs, packets, observations, metrics, verdicts, and artifact refs;
- ad hoc local type aliases are illegal when a canonical shared type exists.

## Audit checks
- registry and local packs agree on type names and roles;
- no duplicate semantic types are introduced casually;
- consumers can reason about bridge types through the registry alone.
