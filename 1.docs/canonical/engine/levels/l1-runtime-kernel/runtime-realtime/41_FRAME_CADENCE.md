# Frame Cadence

## Role

Realtime profile frame cadence contract.

## Data Model

Numeric Authority: STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md §3, §5, §11

Realtime cadence owns the 60 Hz frame path, low-latency queue discipline, visibility freshness, and the profile envelope validated by the integrated realtime fixture.

## Cadence Matrix

| Item | Canonical law | Illegal state |
|---|---|---|
| presentation cadence | fixed 60 Hz | variable presentation contract |
| frame budget | 16.67 ms hard budget | hidden wider frame entitlement |
| presentable queue | <= 1 queued frame | multi-frame latency reservoir |
| visibility freshness | <= 1 stale frame | long-lived stale visibility reuse |

## Failure Posture

- low-value or degradable work yields before low-latency path legality is crossed;
- realtime cadence may not defer queue pressure into later frames to fake stability.


## Operating Matrix

| Concern | Canonical requirement | Illegal posture |
|---|---|---|
| frame ownership | runtime-owned only | service-owned present loop |
| visibility freshness | obey visibility staleness ceiling | using stale visibility beyond law |
| upload timing | bounded by upload law and low-latency path | hidden upload spill into queued frames |
| mixed-pressure proof | must pass `mixed-pressure-realtime-a` | proof by isolated microbench only |

## Queue Pressure Matrix

| Pressure source | Preserve first | Degrade first |
|---|---|---|
| low-latency queue pressure | frame legality | decorative presentation work |
| visibility freshness pressure | near-legal visibility | optional extraction detail |
| upload pressure | low-latency frame path | optional uploads |

## Illegal Path Matrix

| Illegal path | Why illegal |
|---|---|
| hidden second queued frame reservoir | converts latency debt into hidden backlog |
| stale visibility reuse beyond legal window | fakes cadence stability by borrowing correctness |
| upload spill that survives into later frames | turns transfer debt into runtime queue debt |

## Local Operating Law

Frame cadence is runtime-owned and single-path.
No service, presentation policy, or synthesis path may acquire a second cadence governor.
