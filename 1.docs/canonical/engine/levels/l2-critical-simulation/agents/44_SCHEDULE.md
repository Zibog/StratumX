# Schedule

## Role

Bounded schedule and routine progression.

## Schedule Matrix

| Concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| daily schedule cadence | bounded slower cadence | defer low-value routine updates first | per-frame schedule recompute |
| interrupt handling | explicit bounded override classes | clamp overrides before storm | arbitrary schedule invalidation storm |
| persistence/replay | bounded shaped schedule delta records | reduce optional detail first | full schedule dump every tick |

## Ownership Matrix

| Surface | Canonical owner | Allowed output | Illegal output |
|---|---|---|---|
| routine cadence | schedule | bounded next-action window | whole-world routine reevaluation |
| interrupt bridge | schedule + decision bridge | shaped interrupt token only | direct planner invalidation flood |
| replay/persistence | schedule | bounded delta records only | full schedule dump or hidden timer heap export |
| far-agent posture | schedule under tier law | coarse cadence markers | near-fidelity routine ownership at far tiers |

## Failure Matrix

| Pressure class | Preserve first | Degrade first | Illegal fallback |
|---|---|---|---|
| near-agent pressure | current-step legal routine ownership | cosmetic routine detail | schedule shutdown without explicit degradation path |
| interrupt storm | explicit critical override classes | low-value reminders and idle routines | recursive invalidation chain |
| replay pressure | bounded deterministic schedule deltas | optional annotation detail | nondeterministic clock-driven recompute |

## Rule

Schedule remains bounded and shaped for persistence/replay. Interrupt classes may not generate invalidation storms.

## Local Operating Law

Schedule owns bounded routine progression, cadence shaping, and explicit interrupt-class handling.
It may not become a hidden timer heap, a global replanner, or a source of invalidation storms that bypass decision law.
