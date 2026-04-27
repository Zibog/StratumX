# Physics to Destruction and Field Coupling Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes how physics interacts with destruction, terrain response, thermal or wetness fields, atmosphere, and gameplay consequence.

## Coupling law
Physics does not own every downstream consequence.
Physics owns motion, contacts, impulses, and stress-related truth.
It may publish legally typed events that other domains consume.
Destruction, field, material, and gameplay domains own their own consequence truth.

## Coupling classes
| Coupling class | Upstream truth owner | Downstream truth owner | Example |
|---|---|---|---|
| impact_energy_coupling | physics/combat | destruction or material state | bullet or rigid-body hit causing fracture or denting |
| structural_stress_coupling | physics | destruction | overloaded beam breaks after joint stress exceeds threshold |
| thermal_weakening_coupling | field/thermal | destruction/physics policy | heated support loses strength before collapse |
| wetness_or_surface_coupling | field/material | physics/material response | wet surface changes friction or post-impact behavior |
| atmosphere_coupling | field/atmosphere | physics or graphics policy | wind affects exposed debris or particle-facing presentation |
| aftermath_coupling | destruction | cover/navigation/visibility/audio policy | broken wall changes future consequence surfaces |

## Prohibition
Physics may not mutate downstream destruction, fire, atmosphere, or gameplay truth in secret.
It must emit typed consequence triggers or share legally declared substrate state.

## Canonical route examples
### Projectile strikes brittle wall
`combat/kinetics impact -> impact energy classification -> destruction consumes structural consequence trigger -> aftermath published -> graphics/audio/navigation consume aftermath observations`

### Heated support beam fails
`thermal field raises material-state load -> structural threshold policy invalidates support -> destruction publishes break event -> physics rebuilds legal support graph -> aftermath observations published`

### Rain reduces friction and changes stop distance
`weather/wetness field updates material-surface condition -> physics consumes effective friction class -> movement/contact solve produces new body response -> diagnostics may surface degraded assumptions`

## Aftermath law
When a destruction outcome changes future simulation legality, the aftermath is not cosmetic.
It must be promoted into durable downstream truth owners such as cover, visibility, acoustic leakage, navigation, or material-state tables.

## Current posture
The uploaded code proves impact and damage-memory mutation for a narrow slice.
Broad multi-domain aftermath propagation remains `specified_only` or `partial` depending on the target route.
