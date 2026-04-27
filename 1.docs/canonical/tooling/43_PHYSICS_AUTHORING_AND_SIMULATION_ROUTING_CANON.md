# Physics Authoring and Simulation Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document maps physics-facing asks from editor surfaces into tooling intents, preview/runtime classes, diagnostics, and lower-stack consequences.

## Canonical route
`editor physics ask -> tooling physics authoring intent -> transaction or preview classification -> sdk packets only when lower truth must change -> engine physics truth or observation -> diagnostics and editor projections`

## Physics task classes
| Task class | Tooling result class |
|---|---|
| create or edit collider | authoring transaction or denial |
| assign body type, mass, inertia, damping | authoring transaction or denial |
| add or edit constraint | authoring transaction plus debug preview |
| run local physics preview | preview-only result with diagnostics |
| inspect contacts, stress, or sleep posture | diagnostics-only observation |
| hand off to runtime simulation | explicit runtime consequence route |

## Classification law
A physics ask must end as exactly one of:
- authored configuration change;
- preview-only simulation result;
- runtime consequence request;
- diagnostics-only inspection;
- explicit denial/deferment.

## Prohibition
Tooling-local preview state may not be presented as though it already owns engine authoritative body truth.

## Current posture
This canon closes the route language.
General-purpose implementation breadth remains wider than the current uploaded code proof.
