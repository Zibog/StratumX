# Headless Outputs

## Role

Output contract for headless runtime.

## Data Model

Headless outputs are authoritative world deltas, bounded network publications, replay artifacts, storage/verification artifacts, and diagnostics outputs legal under headless ceilings only.
There is no presentable frame path.

## Output Matrix

| Output class | Canonical owner | Ceiling source | Failure posture | Illegal posture |
|---|---|---|---|---|
| authoritative world delta | runtime headless shell | world/replay law | preserve first | presentation-only output |
| bounded network publication | `L2.5` in runtime-owned windows | network ceilings | shed noncritical publication first | unbounded peer fan-out |
| replay/checkpoint artifact | replay law | replay ceilings | reduce optional compare cadence | checkpoint storm |
| storage/verification artifact | startup/runtime law | storage ceilings | defer noncritical verification | hidden persistent IO backlog |
| diagnostics output | diagnostics law | diagnostics ceilings | drop diagnostics first | stealth trace spool |

## Illegal Patterns

- presentable frame ownership in headless mode;
- output-side widening of tick budget;
- hidden debug output growth outside diagnostics law;
- publication queues that outlive profile ceilings.

## Output Window Matrix

| Window class | Required bound | Illegal widening |
|---|---|---|
| authoritative export window | runtime-owned and tick-bounded | service-owned flush window |
| replay artifact window | replay-law bounded | opportunistic per-tick dump |
| storage verification window | startup/runtime bounded | persistent verification backlog |

## Queue Failure Order

When output pressure rises, diagnostics outputs drop first, then optional verification outputs, then noncritical network publication.
Authoritative world deltas and legal replay markers remain last to degrade.

## Local Operating Law

Headless outputs may not create a shadow presentation path, a shadow diagnostics spool, or a long-lived publication reservoir.
All outputs remain subordinate to tick legality.
