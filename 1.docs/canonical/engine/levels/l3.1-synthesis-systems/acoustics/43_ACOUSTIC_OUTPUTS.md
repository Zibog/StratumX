# Acoustic Outputs

## Role

Acoustic output publication contract.

## Output Matrix

| Output class | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| voice mix output | bounded active voice ceiling | drop lowest-value voices first | unbounded voice growth |
| ambience output | bounded ambience reserve | degrade ambience first | ambience steals gameplay reserve |
| streaming audio residency | bounded by acoustics/runtime ceilings | evict optional assets first | acoustics-owned residency governor |
| diagnostics output | diagnostics-law bounded | drop diagnostics first | stealth audio telemetry spool |

## Failure Posture

When acoustics pressure rises, ambience, reflection detail, and optional streaming assets degrade before gameplay-critical voice and bounded publication legality are crossed.

## Output Window Matrix

| Output class | Required bound | Illegal widening |
|---|---|---|
| critical voice mix | bounded active-voice window | hidden voice reservoir |
| ambience publication | bounded ambience reserve | ambience steals gameplay reserve |
| streaming asset publication | runtime-residency bound | acoustics-owned residency law |

## Local Operating Law

Acoustic outputs publish bounded results.
They may not own cadence, residency, or silent multi-buffer growth.
