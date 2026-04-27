# Combat, Ballistics, Damage, and Death Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the canonical lower-stack route for combat consequences.
The route begins with a legal fire action and ends only when impact, damage, vital-state change, death consequence, and diagnostics are all explained.

## Scope
This canon covers:
- fire intent acceptance;
- projectile or hitscan emission law;
- impact against world materials or actors;
- damage consequence routing;
- vital-state changes;
- death/downed/disabled consequence publication;
- diagnostics and observation publication.

This canon does not freeze animation polish, VFX polish, gore, cinematic camera logic, or final AI behavior choreography.

## Domain ownership
| Stage | Owner | Notes |
|---|---|---|
| fire intent | upper stacks request, engine accepts legally | editor/tooling do not own ballistic truth |
| projectile/hitscan execution | engine kinetics/combat | exact execution mode may vary by weapon class |
| impact classification | engine kinetics + world/material/actor contact truth | target class must be explicit |
| damage application | engine damage/vital-state owners | material damage and actor damage are not the same thing |
| lethal/downed/dead consequence | engine vital-state owners | death is runtime truth, not UI convention |
| publication | sdk observations/verdicts + tooling diagnostics | upper stacks inspect; they do not invent consequence |

## Canonical route
`editor or tooling fire ask -> tooling preview/runtime coordination -> sdk ingress packet/control -> engine fire acceptance -> projectile/hit resolution -> target classification -> damage application -> vital-state transition -> sdk observation/verdict -> tooling projection/diagnostic -> editor/runtime surface`

## Target classes
Engine impact routing must classify the target as one of the following:
- `world_static`
- `world_dynamic`
- `destructible_material`
- `actor_body`
- `actor_armor`
- `actor_shield_or_proxy`
- `invalid_or_grazed`

A ballistic route is incomplete if it knows only that "something was hit" but not which target class owns the consequence.

## Damage classes
Damage routing must distinguish at minimum:
- `material_damage`
- `vital_damage`
- `stability_or_stagger_damage`
- `structural_damage`
- `secondary_effect_damage`

Material cracking alone is not proof of actor lethality.
Actor health loss alone is not proof of destruction aftermath.

## Minimum first-kill closure
The first honest combat closure requires at minimum:
1. one active actor or pawn capable of issuing fire intent;
2. one target actor with runtime-owned vital state;
3. legal hit classification from projectile/hitscan to target actor;
4. legal damage application to that actor;
5. a runtime-owned transition to `dead`, `downed`, or another lethal terminal state;
6. diagnostics explaining the consequence.

## Current implementation posture
The uploaded code proves a narrower route:
- a vertical-slice fire ask lowers from Tauri or the desktop host into `VerticalSliceSession`;
- engine ballistics spawns and integrates a projectile;
- wall hit classification is performed;
- impact resolution runs against a material stack;
- damage memory and runtime events are written back into world truth.

That is real and important.
It is not yet full combat closure because actor vital-state and death consequence are not proved in the examined code.

## Required observations
A combat route must be able to publish, directly or by composition:
- shot accepted / denied;
- projectile profile or firing class;
- target classification;
- impact result;
- damage result;
- vital-state result;
- death/downed/disabled result;
- degraded posture if any part of the route fell back.

## Prohibitions
The following are forbidden:
- UI-local target death with no engine consequence;
- material damage memory being reported as actor death;
- assistant/tooling/editor deciding death semantics on their own;
- skipping target classification and directly writing health deltas from the product layer.
