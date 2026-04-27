# Legality Gate and Verdict Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes how unsupported, illegal, deferred, or degraded routes must be reported through the bridge.
It prevents upper stacks from treating every missing route as a generic error.

## Verdict classes
A bridge-level route should be able to distinguish at minimum:
- `accepted`
- `denied_capability`
- `denied_legality`
- `degraded`
- `deferred`
- `execution_error`

## Routing law
Whenever a task is rejected or weakened, the upward route must preserve:
- which capability class was involved;
- whether the problem is lack of capability, active prohibition, or current deferment;
- whether a weaker legal posture exists;
- what diagnostics or facts can be shown to the operator.

## Examples
- `Ballistics integration deferred` is not the same as an execution crash.
It is a recognized route that current implementation postpones.
- `headless profile may not own presentation queue` is a legality denial tied to runtime profile, not a random error.

## Product implication law
Editor and tooling surfaces must not flatten verdict classes into a single friendly string if doing so erases legal meaning.
A denial, degradation, deferment, and execution failure are materially different operator states.
