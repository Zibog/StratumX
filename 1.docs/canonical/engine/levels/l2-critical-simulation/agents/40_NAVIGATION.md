# Navigation

## Role

Bounded navigation solve.

## Navigation Matrix

| Concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| path scope | bounded regional or squad path solve | approximate far routes first | full-world dense path solve |
| rebuild cadence | bounded event/cadence classes | defer low-value recompute first | per-frame full recompute |
| world interaction | staged queries only | fallback deterministically | navigation mutates world truth |

## Rule

Navigation solves only bounded regional or squad path classes. It may not widen into a full-world dense path solve to rescue local cache misses or route churn.

## Planner Matrix

| Planner concern | Canonical posture | Illegal posture |
|---|---|---|
| local route planning | bounded regional/squad planners | dense full-world planner |
| rebuild cadence | explicit classes | per-frame global recompute |
| world interaction | query-only with staged mutation | direct world mutation |

## Local Operating Law

Navigation legality is path-class legality.
No cache miss, route churn, or congestion burst may widen it into global dense planning.
