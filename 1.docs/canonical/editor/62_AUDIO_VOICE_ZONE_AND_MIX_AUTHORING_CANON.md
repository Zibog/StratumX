# Audio, Voice, Zone, and Mix Authoring Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the editor-facing everyday workflow for audio asset use, emitter and listener placement, zone authoring, bus ownership, ducking/priority policy, and runtime-audible diagnostics.
Audio is treated here as a production domain, not as a late debug guest.

## Everyday authoring workflow
`select or import audio asset -> assign event/emitter class -> choose listener profile or preview context -> bind zone and room policy -> bind mix bus / priority / ducking policy -> preview occlusion and variation -> inspect audible result -> commit or revise`

## Required base surfaces
| Surface | Role |
|---|---|
| emitter inspector | placement, attenuation, category, timing class, priority |
| listener profile inspector | gameplay/editor/cinematic/debug listening context, profile differences |
| zone/reverb inspector | room or region policy, dampening, portal behavior, environment relation |
| bus and ducking inspector | route, priority, bus ownership, ducking rules, stolen-voice reasons |
| audio diagnostics panel | audible set, virtualization, ducking, occlusion, denied playback, fallback device/profile |
| variation preview panel | variant pool, repetition guard, timing spread, deterministic preview seed |

## Everyday closure law
The editor everyday surface is not closed unless an operator can do all of the following without falling into deep-lab-only flows:
- author or assign an emitter class;
- author or bind an audio zone;
- select and inspect listener profile;
- preview occlusion;
- inspect active bus and ducking posture;
- inspect variation behavior;
- run an audio regression triplet against a retained baseline.

## Distinction law
Audition is not the same as authoritative runtime playback.
The editor must clearly identify preview-only listening contexts versus committed content state, and must preserve the route back to runtime truth, compare, and capture.

## Disabled posture law
If a preview or commit action is blocked, the first visible denied family must make clear whether the blocker belongs to:
- missing emitter or event class;
- illegal listener context;
- unresolved zone or portal relation;
- bus or ducking policy gap;
- missing baseline or capture target.

## Current posture
This document closes the editor answer layer for everyday audio authoring while broader implementation proof remains future work.
