# Budget Degradation Pressure Recovery And Fallback Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define routing for budget pressure, degrade ladders, old-floor fallback, secondary-viewport throttling, and recovery.

## Intent families
- `intent.budget.inspect_profile`
- `intent.budget.inspect_degrade`
- `intent.budget.inspect_fallback`
- `intent.budget.recover_profile`

## Routing duties
- publish the active profile before showing fallback details;
- show which rung changed first;
- show whether split/detached viewports triggered throttling;
- preserve compare/capture legality verdicts under pressure.

## Recovery law
A recovery route may:
- close secondary viewports;
- reduce preview richness;
- drop optional accelerators.

It may not:
- fake success;
- hide the first degrade cause;
- hide the operator-legality verdict.
