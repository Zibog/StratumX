# Dream Scene Benchmark Workload And Golden Corpus Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the canonical workload packs, golden scenes, and compare expectations for the dream-stack so performance, visual proof, and release readiness are measured against the same retained truth.

## Mandatory workload packs

| Pack | Primary purpose | Required scenes or slices | Primary companions |
|---|---|---|---|
| old-floor tunnel pack | flashlight, shadow ladder, transient light, tunnel audio | tunnel/basement proof region, flashlight sweep, muzzle-flash frame set | engine `137–139`, editor `135`, tooling `90–91` |
| terrain deformation pack | crater, churn, aftermath, terrain material proof | soil, asphalt, rubble, wet ground variants | engine `135`, editor `119/122`, tooling `90–91` |
| destruction block pack | structural support graph, collapse, fragments | wall, slab, beam, facade, roof variants | engine `134`, editor `120`, tooling `90–91` |
| hydrology persistence pack | container inventory, leak, dryout, contamination | barrel, pipe, puddle, overflow slices | engine `112`, editor `121`, tooling `90–91` |
| climate theater pack | distant fronts, storm arrival, visibility | long-range horizon, cloud/depth, precipitation onset | engine `114–115`, editor `136`, tooling `90–91` |
| living/tactics pack | squad intent, cover invalidation, society deltas | localized encounter plus retained social state | engine `120–125`, editor `137`, tooling `90–91` |
| fur/audio continuity pack | microgeometry response and audio continuity | fur/wind/wetness and indoor/outdoor continuity slices | engine `127–128`, editor `125/135`, tooling `90–91` |
| first-playable proof pack | one narrow vertical slice proving the stack | project/world package, corridor or tunnel, material, light, sound, one destruction event | engine `139`, editor `123/127`, tooling `87/90–92` |

## Compare law
- every workload pack must define primary metrics, secondary metrics, and blocker metrics;
- visual packs must name capture camera sets and lighting anchors;
- runtime packs must name performance windows and allowed degrade posture;
- certification blockers must be explicit rather than hidden in prose.

## Required metric classes

| Metric class | Meaning |
|---|---|
| frame budget | frame time or bounded sub-budget requirement |
| visual stability | shadow, lighting, volumetric, or representation stability |
| causality fidelity | whether retained event/summary law remained legal |
| persistence fidelity | whether save/reopen or replay preserves expected state |
| fallback legality | whether the active fallback stayed within declared tier law |

## Golden corpus law
- every workload pack must have one retained golden corpus family;
- golden scenes are immutable until recertified;
- compare tools may not silently change corpus version;
- first-playable proof packs must be small but still use the same corpus law as large packs.

## Companion execution surfaces
- tooling `90–91` own benchmark execution and golden diff routing;
- sdk `82–84`, `88` define compare and proof consumers;
- editor `103`, `124`, `126`, `135`, and `136` own operator-visible review;
- root `114`, `116`, `117`, and `118` must reference these benchmark packs.

## Current posture
`document_gold / workload_and_corpus_closed / execution_open`
