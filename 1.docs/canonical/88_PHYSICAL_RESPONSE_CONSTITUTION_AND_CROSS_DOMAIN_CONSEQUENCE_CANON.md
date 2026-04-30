# Physical Response Constitution And Cross-Domain Consequence Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze the master contour that unifies physical response across materials, rigid bodies, soft bodies, thermal/wetness state, fracture/collapse, ballistics, hydrology, and traversal/nav consequence.
This file does not replace domain canons. It defines the shared constitution they all obey.

## Physical response classes
| Response class | Shared meaning | Typical triggers | Mandatory downstream publications |
|---|---|---|---|
| contact response | immediate force/contact outcome between bodies, terrain, media, or agents | collision, scrape, pressure, resting contact | rigid/soft solve updates, sound class, decal/VFX hint, traversal change if needed |
| rigid response | inertial, constraint, and impact outcome for rigid bodies or rigid terrain sections | collision, support loss, blast impulse, stress | destruction, audio, graphics debris visibility, nav obstruction |
| soft response | deformation, cloth/fur/skin/flexible surface response | wind, drag, contact, deformation, tension | animation/contact solve, audio rustle class, graphics soft-surface state |
| thermal and burn/wet response | temperature, ignition, cooling, wetness, drying, charring | burn, weather, immersion, heat exposure | fire/smoke, material modifiers, audio/media state, graphics overlays |
| fracture and collapse response | break, split, detach, crumble, support cascade | ballistic penetration, blast, fatigue, structural overload | aftermath state, debris, traversal/nav updates, save-state delta |
| ballistic penetration response | projectile flight, impact, penetration, ricochet, wound or structural transfer | weapon fire, shrapnel, fragments | wounds/armor, fracture, audio event class, VFX trace, evidence trail |
| hydrology response | fill, drain, leak, soak, evaporate, flow, pressure transfer | rainfall, leak, immersion, container change | wetness/material modifiers, traversal/media state, persistence delta |
| traversal and navigation consequence | path legality, cover, danger, reservation, locomotion cost | collapse, fire, water, debris, mud, smoke, blocked door | nav graph updates, tactics updates, degraded traversal posture |

## Shared trigger vocabulary
The constitution freezes one common trigger vocabulary for cross-domain reading:
- bullet / fragment / shard impact;
- blast / pressure wave;
- burn / thermal transfer / cooling;
- water / leak / soak / immersion / evaporation;
- stress / fatigue / support loss;
- contact / friction / drag / compression;
- wind / turbulence / media transport.

Domain documents may refine these triggers, but may not invent mutually incompatible top-level vocabularies.

## Solve vs consequence split law
Physical solve is not the same as full consequence.
The split is mandatory:
- pure solve resolves the immediate physical state transition owned by the relevant engine truth families;
- downstream consequence publishes what other systems must now know.

A route is not closed when solve ends. It is closed when downstream consequence is published into the required families.

## Geometry vs material interpretation law
Collision geometry and contact manifold generation are not owned by material law.
Material law owns the interpretation of the event.
The canonical reading order is:
`shape/topology -> contact or penetration candidate -> material stack lookup -> response-profile interpretation -> damage-topology mutation -> downstream publication`.

No domain may reverse this order by pretending that mesh triangles already know fracture, burn, ricochet, or aftermath meaning.

## Damage-topology law
A lawful physical response may mutate compact structural truth without rebuilding arbitrary mesh truth.
The following are first-class legal carriers of physical aftermath:
- damage field patches;
- crack / breach / char topology digests;
- support / occupancy tiles;
- fragment-host absorption state;
- retained world-summary deltas.

A route is legal if these truth carriers are explicit, inspectable, and persistence-safe.
It is illegal if the only record of change is a shader trick or a pile of transient particles.

## Cross-domain publication map
| From physical response | Must publish to |
|---|---|
| contact / rigid / soft | audio event classes, graphics overlays/VFX hints, persistence if state survives |
| thermal / wet / burn | fire/smoke, graphics material modifiers, audio/media state, save/restore deltas |
| fracture / collapse | destruction/aftermath, traversal/nav updates, audio impact/rumble classes, world invalidation |
| ballistic penetration | wounds/armor, fracture, audio/VFX, persistence/evidence bundles |
| hydrology | material modifiers, traversal/media state, persistence, possibly electrical or gameplay consequence if declared elsewhere |
| traversal/nav consequence | tactics/AI, legality, degraded posture, compare/capture if proof-relevant |

## Material law relation
Material-first truth remains the governing interpretation of physical response.
Every physical event must be readable against:
- material archetype;
- surface family;
- territory family;
- response profile;
- active modifiers;
- consequence tier;
- sleep/wake posture.

No terrain/object/structure/living split may create separate incompatible material languages.
Terrain surface family, prop surface family, structural surface family, vegetation surface family, and living-surface family must remain readable through one master material law.

## Persistence and world law
A physical response with surviving aftermath is not complete until world and persistence law decide whether it becomes:
- transient only;
- local chunk delta;
- world package state;
- restore-safe baseline change;
- validation blocker.

## Prohibitions
- No domain may treat bullet, blast, burn, water, stress, temperature, or contact as unrelated local metaphors.
- No physical route may stop at solver output if downstream consequence families changed.
- No terrain truth, mesh truth, structure truth, and living-surface truth may diverge into incompatible response vocabularies.
- No persistence layer may save aftermath without world-family legality and identity.

## Required companion docs
- `19_PHYSICS_CAPABILITY_MATRIX.md`
- `40_TECHNOLOGY_ORCHESTRA_CANON.md`
- `49–73` engine physics/world/systemic consequence docs
- `84–87` material-first closure docs
- `96–98` material runtime closure docs
- `world/50–56`
