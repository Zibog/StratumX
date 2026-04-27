# Audio Streaming, Mix, and Runtime Resolution Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes how audio assets, streams, buses, voice budgets, ducking, and runtime resolution behave under engine ownership.

## Scope
This canon covers:
- streaming versus resident audio policy;
- bus and subgroup routing;
- world/UI/music/voice separation;
- voice limits and virtualization;
- ducking and focus policy;
- runtime degradation under budget pressure.

## Mix law
Mix policy is engine runtime policy, even when authored above engine.
Authoring surfaces may define intended routes and categories, but the runtime decides the active mix outcome under budget and listener posture.

## Streaming law
Large audio assets may stream.
Streaming policy must declare residency class, prefetch expectations, fallback behavior, and denial posture when budgets are exceeded.
The stack may not silently pretend a streamed asset is resident when it is not.

## Bus classes
| Bus class | Examples |
|---|---|
| world_bus | emitters in the world, ambience, impacts, weather |
| voice_bus | dialogue, radio, NPC or system voice |
| ui_bus | menu, editor, notification, command feedback |
| music_bus | score and adaptive music |
| diagnostics_bus | optional trace or debug-only audible surfaces |

## Priority law
Voice or emitter priority must be explicit.
When virtualization or culling occurs, diagnostics must be able to name the reason.

## Current posture
The canon is now explicit about audio runtime resolution, but the uploaded code sample does not yet prove a broad engine-backed route.
Current posture remains `specified_only`.
