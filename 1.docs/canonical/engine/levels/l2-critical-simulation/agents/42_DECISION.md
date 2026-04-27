# Decision

## Role

Bounded decision solve.

## Decision Matrix

| Concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| planner class | squad-first and bounded local planners | degrade low-value agents first | monolithic global planner |
| mutation | stage through world/apply law only | defer optional side effects | direct world mutation |
| remote burden | bounded by profile and tier law | preserve local near reserve first | remote decision steals local reserve |

## Planner Matrix

| Planner class | Canonical posture | Illegal posture |
|---|---|---|
| bounded local planner | legal | monolithic global planner |
| staged mutation | legal | direct world mutation |
| remote burden | bounded by profile | remote decision steals local reserve |

## Coordination Ownership Matrix

| Surface | Canonical owner | Legal bridge | Illegal bridge |
|---|---|---|---|
| local tactical solve | decision | bounded action tokens | cross-world planning graph |
| squad coordination | decision | shaped squad directives | hidden persistent coordination state |
| schedule integration | schedule + decision bridge | explicit schedule token consumption | direct schedule invalidation ownership |
| world mutation | world/apply | staged authoritative deltas only | planner-owned state writes |

## Failure Matrix

| Pressure class | Preserve first | Degrade first | Illegal fallback |
|---|---|---|---|
| near-agent combat or hazard pressure | critical local solve | low-value idle planners | whole-population global planner |
| host/headless remote burden | local authority legality | distant/low-value planning detail | remote-first planning inversion |
| mutation pressure | staged delta legality | optional side effects | direct mutation fast path |

## Rule

Decision uses bounded planners only. Monolithic global planning and direct world mutation are illegal.

## Local Operating Law

Decision owns bounded planning, explicit coordination scope, and staged authoritative outputs only.
It may not create hidden coordination state, direct-write world state, or bypass world/apply law under pressure.
