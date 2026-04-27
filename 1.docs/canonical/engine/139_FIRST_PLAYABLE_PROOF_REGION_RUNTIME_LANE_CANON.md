# First Playable Proof Region Runtime Lane Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the runtime obligations for the first narrow proof region.

## Required subsystems

| Subsystem | Runtime obligation |
|---|---|
| world package open | one lawful world package with ring and anchor validity |
| terrain/material visibility | one honest terrain and material corridor visible in runtime |
| sky/time/weather binding | one sky/time/weather state affecting the region |
| proof pocket | one tunnel or basement pocket with light, sound, and visibility stress |
| destruction and aftermath | one runtime destruction event plus aftermath publication |
| audible material loop | one step or impact loop tied to material law |
| capture-ready frame chain | one retained artifact path for proof and compare |

## Runtime phases
`open world -> bind corridor -> bind sky and weather -> prepare proof pocket -> run light/audio/destruction slice -> capture -> publish readiness verdict`

## Forbidden shortcuts
- no fake splash viewport instead of terrain plus sky frame;
- no offline-rendered tunnel clip instead of runtime proof;
- no silent fallback without a published verdict;
- no capture green without artifact refs.

## Failure families
- `proof_lane.world_not_open`
- `proof_lane.sky_not_bound`
- `proof_lane.proof_pocket_missing`
- `proof_lane.capture_not_ready`

## Current posture
`document_gold / runtime_lane_closed / implementation_open`
