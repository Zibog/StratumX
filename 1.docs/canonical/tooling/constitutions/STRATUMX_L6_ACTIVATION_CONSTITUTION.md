# STRATUMX_L6_ACTIVATION_CONSTITUTION

## Scope
This constitution governs activation and hot/cold posture in tooling.

## Binding laws
- activation state is explicit, bounded, and auditable;
- inactive tools/services go cold unless explicitly justified otherwise;
- activation rules and activation state remain separate concerns.

## Audit checks
- activation sidecars state triggers, priorities, and fallback modes concretely;
- no always-live hidden subsystem bypasses activation law;
- budgets and invalidation rules respect activation state.
