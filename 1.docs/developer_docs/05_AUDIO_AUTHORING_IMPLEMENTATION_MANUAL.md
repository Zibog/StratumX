# Audio Authoring Implementation Manual

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **developer guide**.


## Goal

Implement authored audio events, bank cook, material sound matrix, audition, and diagnostics.

## Implementation order

1. audio event identity;
2. event graph normalized nodes;
3. variation sets;
4. material sound row registry;
5. bank manifest and stream chunks;
6. audition route;
7. occlusion trace route;
8. mix diagnostic route;
9. explain silence route;
10. editor lab integration.

---

# V34 audio backend strategy and authoring closure

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Binding decision

StratumX owns audio truth. Audio device libraries, platform backends, middleware adapters, and low-level mixers are replaceable.

## 1.0 audio target

1.0 requires:

- event authoring;
- material sound matrix;
- sample/layer variation;
- bank cook;
- bus/mix route;
- occlusion/zone placeholder;
- editor audition;
- no-sound diagnostics;
- capture/evidence.

1.0 does not require final AAA propagation, full convolution, final neural voice, or platform-specific audio middleware.

## Replaceable backend law

| Layer | Owner |
|---|---|
| Audio event truth | StratumX engine |
| Material sound matrix | StratumX material/audio law |
| Bank/cook metadata | StratumX tooling |
| Editor audition workflow | StratumX editor |
| Device output | replaceable backend |
| Middleware adapter | optional, never canonical truth |

## Minimum graph nodes

- Event Input
- Parameter Input
- State Switch
- Weighted Random
- Layer Mixer
- Material Resolver
- Occlusion Resolver
- Bus Send
- Loop/Tail Controller
- Debug Probe
- Output

## Material sound matrix example

| Material family | Step | Impact | Scrape | Break | Rain | Fire |
|---|---|---|---|---|---|---|
| wet_mud | squish variants | dull thud | smear | none | splash loop | steam/suppress |
| dry_soil | crunch variants | dirt hit | scrape dust | crumble | patter | crackle if dry |
| asphalt | hard step | sharp hit | scrape | chip/spall | wet slap | low burn none |
| wood | creak step | knock | scrape | crack/splinter | damp knock | crackle/char |
| metal | ring step | clang | screech | deform | ping | heat tick |

## Acceptance

A source sample becomes runtime-ready only after:

1. format decode;
2. loudness verdict;
3. loop/tail validation;
4. bank identity;
5. event binding;
6. material matrix binding if surface-driven;
7. bus route;
8. missing sample fallback;
9. audition result;
10. no-sound troubleshooting route.
