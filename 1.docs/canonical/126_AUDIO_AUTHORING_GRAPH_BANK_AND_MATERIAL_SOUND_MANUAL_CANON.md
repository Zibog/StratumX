# Audio Authoring Graph, Bank, Material Sound, Occlusion, and Audition Manual

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **root canonical manual**.


## Purpose

This document completes StratumX audio as an authored and diagnosable system, not merely a runtime emitter chain.

## Audio authoring objects

| Object | Meaning |
|---|---|
| Audio event | Named playable event with routing, layers, variants, and runtime parameters. |
| Event layer | A component of an event: transient, loop, tail, sweetener, impact, scrape, mechanical, weather, voice. |
| Variation set | Stochastic or state-driven choices that prevent repeated samples from sounding identical. |
| Material sound row | Surface/material × event type × state row for steps, impacts, scrapes, breaks, rain, fire, fluid. |
| Bank | Cooked collection of audio assets, metadata, stream chunks, and event definitions. |
| Bus | Mix destination with priority, ducking, gain, effects, and diagnostics. |
| Portal/zone | Acoustic region, occlusion portal, indoor/outdoor transition, reverb or coloration scope. |

## Authoring graph baseline

The authoring graph must support event input, parameter input, state switch, random/weighted variation, layer mixer, bus send, material resolver, occlusion resolver, tail/loop control, and debug probe nodes.

## Bank pipeline

Source audio becomes release-grade only after loudness normalization, loop/tail validation, bank identity, stream chunking, memory tier classification, missing sample fallback, and audition proof.

## Material-first sound law

Footsteps, impacts, scrapes, bullet hits, debris falls, rain, fire, water leaks, cloth movement, and destruction sounds must resolve through material/surface state rather than object-name shortcuts.

## Editor audition

The editor must allow auditioning event variants, material rows, occlusion scenarios, indoor/outdoor portals, weather state influence, and mix bus priority.

## Non-negotiable rules

| Rule | Meaning |
|---|---|
| No fake success | A route may return `NotImplemented`, `Blocked`, `Unavailable`, or `Unsupported`, but it may not return success for an unimplemented behavior. |
| One owner | Every truth object has exactly one owner layer. Other layers may hold handles, DTOs, views, or cached projections only. |
| Observable failure | Every failure family must produce an error code, disabled reason, recovery hint, and retained diagnostic packet. |
| Editor honesty | The editor may expose the route, preview, capture, or recovery action, but it must not mutate engine truth except through lawful SDK/tooling ingress. |
| Evidence or it did not happen | Release-grade claims require capture, compare, replay, or retained diagnostic evidence. |

## Required completion shape

Every implementation derived from this document must include:

1. owner module or crate;
2. public data contracts;
3. lifecycle stages;
4. failure and disabled reason codes;
5. editor/tooling/SDK contact points;
6. quality tests and negative-path tests;
7. retained evidence artifacts for certification routes.

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
