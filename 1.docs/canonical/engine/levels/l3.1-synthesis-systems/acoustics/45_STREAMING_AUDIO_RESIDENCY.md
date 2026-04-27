# Streaming Audio Residency

## Role

Audio streaming residency use.

## Data Model

Streaming audio residency consumes runtime-managed pages and acoustics-owned buffer ceilings.
It may not create a parallel residency governor.

## Residency Matrix

| Item | Canonical law |
|---|---|
| streaming audio pages | owned by stream/residency law |
| decode concurrency | bounded by numeric source |
| active page pressure | bounded by acoustics/runtime ceilings |
| ambience eviction | legal before critical cue eviction |

## Canonical Law

- streaming audio pages remain under stream/residency/transfer law;
- decode concurrency and active pages remain bounded by numeric source;
- acoustics may evict ambience before critical cues under degradation law.
