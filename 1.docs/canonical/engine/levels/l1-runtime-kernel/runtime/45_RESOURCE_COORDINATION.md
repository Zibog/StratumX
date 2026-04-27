# Resource Coordination

## Role

Runtime coordination surface for `L1.5` and synthesis/resource service windows.

## Coordination Matrix

| Concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| service wakeups | runtime-owned phase windows only | defer noncritical service work | service-owned wakeup loop |
| queue coordination | queue depth and queue age ceilings both enforced | drop or defer optional class work | hidden queue reservoir |
| transfer coordination | upload/readback obey transfer law and low-latency path | shed optional upload first | service widens frame queue |
| visibility/resource coupling | imaging may read bounded resource state only through runtime windows | drop optional extraction detail | synthesis-owned residency governor |
| startup warm remnants | may not survive as post-launch governors | discard optional warm artifacts | startup-owned persistent coordinator |

## Illegal Patterns

- secondary scheduler hidden in service coordination;
- queue-depth compliance with illegal queue age;
- resource coordination that persists as a second owner after bootstrap.

## Coordination Table

| Binding | Canonical source |
|---|---|
| queue depth and age ceilings | `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` |
| service wakeup legality | `STRATUMX_EXECUTION_CONSTITUTION.md` |
| transfer/upload legality | `STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md` |
| startup handoff | `levels/l4-startup/startup/42_RUNTIME_WIRING.md` |

## Failure Posture

When coordination pressure rises, optional service work, optional uploads, and optional warm artifacts yield before runtime-owned progression or queue legality is crossed.

## Escalation Order

1. drop diagnostics-only work;
2. defer optional warm or maintenance activity;
3. degrade decorative synthesis work;
4. clamp noncritical publication;
5. preserve world/apply, legal transport, and replay cadence.

## Local Operating Law

Resource coordination is the runtime choke point for stalls, queue creep, and latency rot.
No local service document may replace it with independent per-service arbitration.
