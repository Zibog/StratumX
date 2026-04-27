# World Validation And Degraded Posture Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze world validation as a first-class law covering legality, persistence readiness, chunk closure, linkage closure, and degraded posture.

## Validation scope
World validation must check at minimum:
- identity and namespace continuity;
- chunk invalidation and rebuild closure;
- streaming/residency legality;
- material registry linkage closure;
- environment binding closure;
- save/restore readiness;
- current degraded posture and whether it remains lawful for the current route.

## Degraded posture law
A world degraded posture must remain explicit.
It must publish:
- which capability scope was reduced;
- which chunks/assets/systems are affected;
- whether the reduced posture is still legal for authoring, simulation, compare, or release;
- the first legal recovery action.

## Deny conditions
- validation green while mandatory blockers remain unresolved;
- undeclared degraded posture affecting authoring or release proof;
- missing recovery action for a degraded but recoverable state.


## Validation inputs expanded
World validation explicitly includes:
- chunk integrity and rebuild posture;
- terrain/material linkage legality;
- sky/environment binding legality;
- placement/linkage legality for world-authored sources and zones;
- save/restore identity consistency.

## Retained verdict law
`btn.world.validate_world` must produce a retained verdict artifact or a retained blocker trace.
A generic pass/fail string is not sufficient.

## Material-centric degraded validation
World validation must fail when a world references a material instance that:
- lacks required branch coverage;
- binds illegal cheap-runtime rungs for declared hardware/runtime floors;
- stores non-canonical branch-local aliases instead of normalized material-facing refs.
