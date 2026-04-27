# Scheduler

## Role

Runtime-owned scheduler authority.

## Data Model

The scheduler owns worker activation, phase barriers, queue arbitration, and legal parallel execution order.
No other crate may create a competing global scheduler.

## Canonical Scheduler Law

| Item | Canonical law |
|---|---|
| global scheduling ownership | `engine_runtime` only |
| queue ceilings | obey numeric-source depth and age ceilings |
| work stealing | legal only inside runtime-owned scheduler law |
| broad locks across compute phases | illegal |

## Illegal Patterns

- nested global scheduler inside a family or service crate;
- queue growth hidden behind helper abstractions;
- barrier insertion outside runtime law.
