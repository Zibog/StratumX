# Audio Emitter, Listener, and Occlusion Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes runtime audio truth for emitters, listeners, spatial zones, occlusion, obstruction, reflection classes, and audible-priority routing.

## Truth law
Audio runtime truth lives in engine-owned state.
Editor, tooling, and sdk may author configuration and consume observations, but may not own hidden audible truth.

## Core audio classes
| Class | Role |
|---|---|
| emitter | world- or UI-origin sound source |
| listener | active hearing frame for a view, player, cinematic, or debug probe |
| acoustic_zone | region-scoped policy such as reverb, dampening, or portal behavior |
| occlusion_policy | legal attenuation or filtering from blocked paths |
| obstruction_policy | near-path or partial blocking response |
| priority_or_virtualization_policy | budget-aware audible scheduling |

## Canonical route
`emitter or system event -> listener-context selection -> occlusion/zone evaluation -> mix-class resolution -> priority/virtualization decision -> published audible set and diagnostics`

## Listener law
A sound answer is incomplete if it does not identify which listener or hearing context owns the result.
Gameplay listener, editor preview listener, cinematic listener, and debug listener are distinct legal contexts.

## Zone law
Zones may modulate or route audio, but they do not invent emitter truth.
They are policy surfaces over already legal emitters and listeners.

## Diagnostics law
The stack must be able to explain whether a sound is:
- audible;
- virtualized;
- ducked;
- occluded;
- obstructed;
- zone-routed; or
- denied by budget posture.

## Current posture
Audio is strongly specified in canon but not broadly proved in current uploaded code.
Current posture is `specified_only`.
