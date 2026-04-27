# Agent Damage, Reaction, and Vital-State Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the runtime truth model for actor damage and mortality consequences.
It bridges the gap between "a projectile hit something" and "a live actor changed state in a way the world and editor can inspect honestly".

## Required state classes
At minimum, engine truth must support the following runtime-visible classes for a damageable actor:
- `alive`
- `hurt`
- `staggered_or_destabilized`
- `downed`
- `dead`
- `disabled_nonlethal`

A game may add more classes.
It may not silently skip this ladder and call every consequence merely "damage".

## Damage intake law
Actor damage intake must have explicit ownership over:
- source classification;
- hit zone or hit proxy class when relevant;
- raw damage signal;
- mitigation/armor/posture modifiers;
- resulting vital-state delta;
- resulting observable consequence.

## Reaction law
Reaction is downstream of damage and vital-state, not upstream.
The runtime may choose no visible animation reaction for a frame, but it may not therefore omit the underlying state transition.

## First honest lethal closure
The first honest lethal route is satisfied when one actor can transition from `alive` to `dead` through a runtime-owned damage consequence and the editor/tooling stack can inspect:
- who was hit;
- by what class of attack;
- what vital-state delta occurred;
- what terminal state was reached.

## Current implementation posture
The uploaded code examined for this patch did not prove a live actor vital-state route.
Actor authoring DTOs exist in tooling session code, but that is not runtime-owned mortality proof.
Current posture: `specified_not_proved`.
