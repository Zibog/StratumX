# Interaction to Runtime Consequence Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the product-layer answer to asks like:
- move something;
- open something;
- fire something;
- trigger a world consequence;
- inspect what really happened.

## Interaction law
An interaction is not complete when the button or gizmo responds.
It is complete only when the stack can explain whether the interaction:
- changed authoring truth only;
- previewed a possible result;
- changed runtime/world truth; or
- was denied/deferred.

## Canonical route
`editor interaction surface -> tooling intent or preview/runtime coordination -> sdk packet/control surfaces when lower truth is needed -> engine consequence -> upward observations/diagnostics -> editor confirmation surfaces`

## Task classes
| Interaction class | Typical asks | Current posture |
|---|---|---|
| authoring transform interaction | move, rotate, scale | partial |
| scene-local structural interaction | add/remove entity, bind stack | partial |
| preview/runtime interaction | enter play/simulate, pause, step | partial |
| combat interaction | fire shot, inspect hit | partial to live depending on demo scope |
| world/system interaction | ignite, fracture, weather change | specified_only |
