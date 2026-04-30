# Sound to Speaker Delivery Chain Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze the orchestration-grade chain from runtime event and world state to speaker output.
This document makes audio a first-class domain: event truth, propagation, occlusion, zone and mix policy, fallback, diagnostics, and release evidence must all read as one chain.

## Canonical chain
`event emission -> event classification -> emitter/listener resolution -> propagation and occlusion -> zone/reverb/context resolution -> priority/ducking/voice ownership -> streaming and decode readiness -> mix bus ownership -> device/output presentation -> diagnostics/capture`

## Exact handoff boundaries
| Stage | Primary owner | Mandatory inputs | Mandatory outputs | First-class failure posture |
|---|---|---|---|---|
| event emission | engine `93`, gameplay/domain owners | authored event, runtime event trigger, world context, category | event identity, timestamp, source identity, category | deny if event identity, source, or category is illegal |
| classification and routing intent | engine `93`, editor `62` | event class, subtitle/voice/music/world/UI role, mix intent | routing class, candidate bus family, priority class | silent result may not be hidden inside bad classification |
| emitter and listener resolution | engine `57`, `93` | emitter profiles, listener profiles, active view/listener context | emitter-listener pairs, attenuation scope, listener profile verdict | operator must see whether the wrong listener context suppressed playback |
| propagation and occlusion | engine `57` | geometry, materials, portals, distances, media state | propagation path, occlusion chain, obstruction verdict | muffled/silent results must cite the first occlusion or obstruction boundary |
| zone, reverb, and environment context | engine `57`, editor `62` | zone volumes, portals, room class, weather and media context | zone assignment, reverb/room profile, environmental coloration | no zone coloration may be magic or editor-local |
| priority, ducking, and voice ownership | engine `58`, `93` | bus budgets, audible set, dialogue/music/UI rules, concurrency limits | audible set, suppressed set, ducking state, stolen-voice reasons | voice steal or ducking must publish the first winning rule |
| streaming and decode readiness | engine `58` | asset refs, stream residency, codec readiness, warmup posture | decode/stream verdict, fallback clip, active degrade rung | late or absent audio must cite stream or decode readiness explicitly |
| mix bus ownership | engine `58`, `93` | bus graph, sends, gains, filters, listener context | mix graph result, bus ownership verdict, peak/load posture | every bus mute, clip, or deny must resolve to one owner decision |
| output device and presentation | runtime platform chain | final bus output, device profile, fallback device | speaker/headphone/device verdict, output format, fallback state | silent device output must remain attributable after mix succeeds |
| diagnostics and capture publication | engine `84`, editor `95`, `103`, `105`, `109` | traces, compares, captures, baseline refs | audibility reason chain, compare digest, evidence bundle | no audio route is closed if silence cannot be reproduced and captured |

## Everyday audio authoring closure
The base operator surface must be able to author and inspect at least:
- zone authoring;
- emitter class assignment;
- listener profile selection;
- occlusion preview;
- bus inspect;
- variation inspect;
- ducking inspect;
- audio regression triplet.

These are not luxury labs. They are the minimal product surface for treating audio as a real production domain.

## Truth split law
Audio truth must stay split and explicit across the following families:
- sound event truth;
- emitter/listener truth;
- propagation/occlusion truth;
- zone/reverb/environment truth;
- mix/priority/ducking truth;
- device/output truth.

No single “audio state blob” may replace these owned families.
No editor preview path may invent a hidden mix or listener context that cannot be diagnosed or replayed.

## Subtitle and voice relation law
If subtitle, dialogue, voice priority, or accessibility cues participate in release-critical output, the route must declare whether the audio event:
- published subtitle/voice lineage correctly;
- was suppressed but still legally subtitled;
- was ducked or stolen by a higher-priority event;
- fell back to device/profile-safe output.

## Old-floor and degraded posture law
Audio old-floor proof must retain:
- active stream/decode/device degrade rung;
- suppressed or virtualized set;
- first winning ducking or priority rule;
- fallback output device/profile if selected;
- compare digest against the last-good baseline.

## Prohibitions
- No sound route may stop at “event fired” and call the chain closed.
- No silent result may require guessing whether the cause was event, routing, occlusion, streaming, mix, or device output.
- No audio preview may become release authority if it cannot publish compare and capture artifacts.
- No device fallback may be hidden from the operator.

## Required companion docs
- `21_AUDIO_CAPABILITY_MATRIX.md`
- `40_TECHNOLOGY_ORCHESTRA_CANON.md`
- `57_AUDIO_EMITTER_LISTENER_OCCLUSION_CANON.md`
- `58_AUDIO_STREAMING_MIX_AND_RUNTIME_RESOLUTION_CANON.md`
- `93_AUDIO_EVENT_CLASSIFICATION_EMISSION_AND_TIMING_CANON.md`
- `editor/62_AUDIO_VOICE_ZONE_AND_MIX_AUTHORING_CANON.md`
- `editor/95_AUDIO_RUNTIME_AND_MIX_LAB_CANON.md`

---

# V33 authoring-to-speaker closure

Stack version: `SX-CANON/1.0.27/STACK-v33`

## Authoring-to-speaker lifecycle

A sound heard by the player must be traceable through:

1. source asset or generated event;
2. audio event definition;
3. material/state variant selection;
4. emitter/listener relationship;
5. occlusion/portal/weather/world influence;
6. priority and mix bus routing;
7. bank/stream residency;
8. device output;
9. diagnostic capture and editor audition proof.

No audio route is release-grade without a material/event matrix row, bank identity, fallback sound, and failure packet.
