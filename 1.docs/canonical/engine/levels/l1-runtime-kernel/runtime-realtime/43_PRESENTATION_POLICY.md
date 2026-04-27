# Presentation Policy

## Role

Presentation-side legality for realtime runtime.

## Presentation Policy Matrix

| Concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| queued presentable frames | <= 1 by numeric source | degrade decorative work first | latency reservoir |
| low-latency mode | zero avoidable queue growth | drop optional quality first | hidden buffering |
| visibility freshness | obey visibility staleness ceiling | accept quality degradation before stale-frame breach | stale visibility reuse beyond law |
| driver/backend posture | fixed to capture-sheet or runtime fallback contract | refuse unsupported proof posture | backend drift during proof |
| degradation order | obey profile degradation law | runtime-owned degradation only | imaging/audio chooses its own priority order |
| diagnostics readback | explicit and bounded | drop diagnostics first | hidden readback governor |

## Rule

Presentation policy may reduce decorative quality but may not borrow world, network, replay, or residency reserve.

## Illegal Patterns

- queue growth used as a hidden stability valve;
- stale visibility accepted to hide upload or extraction debt;
- presentation policy overriding profile degradation order;
- presentation-owned residency or transfer governance.

## Binding Table

| Binding | Canonical source |
|---|---|
| queue and queue-age ceilings | `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` |
| visibility staleness | `STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md` |
| degradation order | `STRATUMX_DEGRADATION_POLICY_LAW.md` |
| diagnostics readback posture | `STRATUMX_DIAGNOSTICS_CEILING_LAW.md` |

## Illegal Path Matrix

| Illegal path | Why illegal |
|---|---|
| presentation-owned residency governor | steals `L1.5` ownership |
| stale-frame acceptance to hide upload debt | converts transfer failure into correctness drift |
| diagnostics readback in the critical frame path | spends frame budget on noncritical work |

## Priority Matrix

| Pressure source | Preserve first | Degrade first |
|---|---|---|
| queue depth pressure | one-frame legality | decorative presentation quality |
| visibility pressure | near visibility correctness | far decorative fidelity |
| diagnostics pressure | frame path legality | diagnostics readback |

## Local Operating Law

Presentation policy may choose what to lose first, but never what the runtime is allowed to own.
Queue legality, degradation order, and residency ownership remain runtime-law surfaces.
