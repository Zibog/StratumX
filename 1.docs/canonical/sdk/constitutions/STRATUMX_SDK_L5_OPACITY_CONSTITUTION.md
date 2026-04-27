# STRATUMX_SDK_L5_OPACITY_CONSTITUTION

## Scope
This constitution governs opaque values carried by the bridge.

## Binding laws
- opaque handles/refs may be routed and compared but not structurally decoded by consumers;
- dereference must happen through declared surfaces;
- opacity never excuses missing type class, scope, or freshness metadata.

## Audit checks
- registry entries classify opaque values clearly;
- no file relies on payload-shape guessing;
- opacity is preserved without sacrificing auditability.
