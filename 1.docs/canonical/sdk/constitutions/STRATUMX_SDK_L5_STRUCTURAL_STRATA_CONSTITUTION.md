# STRATUMX_SDK_L5_STRUCTURAL_STRATA_CONSTITUTION

## Scope
This constitution fixes the structural strata inside `L5`.

## Binding laws
- packets/controls, observations/metrics, compatibility/verdicts, policies, and opaque refs are distinct strata;
- a lower stratum may support an upper one, but the bridge may not collapse all strata into one untyped bucket;
- level-local docs must reflect the declared strata map.

## Audit checks
- every `L5.x` level sits clearly in a stratum;
- cross-stratum references are explicit;
- structural compression does not hide semantic class boundaries.
