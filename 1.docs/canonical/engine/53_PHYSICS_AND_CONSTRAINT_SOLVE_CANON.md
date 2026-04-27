# Physics and Constraint Solve Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the legal runtime model for collision, body motion, contact persistence, constraints, impulses, sleep/awake transitions, and deterministic solve ordering.

## Scope
This canon covers:
- collision representation and contact generation;
- rigid or kinematic body state;
- force, impulse, and velocity integration;
- constraint families such as fixed, hinge, slider, spring, and character-support constraints;
- sleep, wake, and stabilization policy;
- deterministic ordering and replay boundaries.

It does not cover destruction aftermath, fire coupling, or gameplay consequence law except where needed to define solve boundaries.
Those routes are downstream in `54`, `50`, `59`, root `88`, and root `97`.

## Truth law
Physics truth lives in engine-owned runtime state.
Editor, tooling, and sdk may request configuration or consume observations, but they do not own runtime contact truth, solver truth, or authoritative body state.

## Core body classes
| Class | Role | Legal authority |
|---|---|---|
| static_body | immovable world support | authored configuration, runtime-readable |
| kinematic_body | externally driven legal motion with collision participation | authored/control-driven targets, engine resolves contacts |
| dynamic_body | fully simulated body | engine runtime truth only |
| trigger_volume | overlap-only semantic volume | engine runtime truth with authored shape/config |
| character_support_proxy | locomotion-support abstraction | engine runtime truth with control-driven intent |

## Solve ordering law
The canonical step order is:
`input/control consequences already resolved -> broadphase candidate generation -> narrowphase contact generation -> contact persistence refresh -> force accumulation -> constraint solve -> velocity/position integration -> sleep/stabilization pass -> publication of physics observations`.

No upper stack package may assume body truth before publication.

## Geometry-first contact / material-first interpretation law
Physics owns contact manifold generation.
Material law owns the meaning of the contact.
The canonical order is:
- geometry and bounds produce broadphase candidates;
- narrowphase produces contact manifolds or penetration candidates;
- material package lookup interprets friction, restitution, penetration, scrape, fracture, or support consequences;
- downstream systems consume the published digests.

A mesh, collider, or body may not hide material-specific fracture or ricochet law inside the manifold generator.

## Constraint law
A constraint definition may be authored above engine, but constraint state is always engine truth.
A constraint must declare:
- anchor frames;
- allowed axes or degrees of freedom;
- limits;
- break thresholds if any;
- damping/compliance class;
- whether it is gameplay-critical or cosmetic.

## Determinism law
Physics is required to be deterministic within the declared platform and precision envelope for the same input stream, asset state, and budget tier.
Where full determinism is not legal across a degradation or precision boundary, the engine must publish an explicit degraded posture rather than silently pretending equivalence.

## Sleep and wake law
Bodies may sleep only under explicit engine-owned criteria.
Any authored or control-driven event that invalidates a sleeping assumption must wake the body and publish the state transition.
Upper layers may request wake intent but may not directly toggle hidden runtime sleep truth.

## Publication law
Published physics observations may include:
- contact summaries;
- penetration or overlap diagnostics;
- constraint-stress diagnostics;
- body motion summaries;
- sleep/wake events;
- degraded solver posture;
- material-pair interpretation digest when the contact produced downstream consequence.

Observations are never authority over the runtime state they report.

## Current implementation posture
Current uploaded code proves a narrow ballistic/impact runtime path and broader physics/destruction intent at the canon level.
A complete general-purpose body/constraint authoring and inspection route is therefore `specified_only` at this wave.
