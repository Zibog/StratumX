# L5 Export Epoch And Invalidation

## Role

Startup-owned epoch and invalidation model for public bridge export surfaces.

## Epoch Matrix

| Concern | Required law | Illegal posture |
|---|---|---|
| bridge epoch identity | explicit and monotonic per launched runtime instance | hidden epoch reuse |
| snapshot invalidation | explicit and externally visible | silent mutable replacement |
| batch continuity | explicit per source surface | undeclared cursor break |
| profile transition | requires a new legal launch or explicit restart path | post-launch export widening |
| runtime death or drain | explicit invalidation signal | stale public bind treated as live |

## Rule

Public bridge export surfaces must publish a monotonic epoch and explicit invalidation posture.
External `L5` readers may rely on those signals to detect snapshot replacement, cursor invalidation, and runtime death, but may not infer hidden internal ownership.

## Failure Posture

If epoch continuity, invalidation visibility, or runtime liveness cannot be preserved, the public bridge bind must fail closed or roll to a new explicit bridge epoch.

## Illegal Patterns

- silent snapshot swap without invalidation;
- reused dead-runtime bridge epochs;
- stale exported handles surviving past declared runtime death;
- cursor continuity implied without a declared epoch law.

## Local Operating Law

Bridge epochs make the public bind auditable.
They prevent external readers from guessing whether exported surfaces still describe the live engine instance.
