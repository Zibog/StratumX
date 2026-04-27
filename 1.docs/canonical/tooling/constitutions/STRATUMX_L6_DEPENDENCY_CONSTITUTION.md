# STRATUMX_L6_DEPENDENCY_CONSTITUTION

## Scope
This constitution constrains legal dependency edges inside `L6`.

## Binding laws
- `L6` depends downward on `sdk/L5` and global umbrella authorities;
- families and sidecars may depend only on declared planes/services/registries;
- editor product semantics may be consumed as client surfaces, not as hidden dependencies owned by tooling.

## Audit checks
- local dependency docs name concrete legal surfaces;
- no sidecar reaches across undeclared plane boundaries;
- upper campaign/assistant logic does not leak downward into core tooling dependencies.
