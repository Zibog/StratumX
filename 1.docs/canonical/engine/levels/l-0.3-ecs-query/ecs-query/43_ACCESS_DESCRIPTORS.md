# Access Descriptors

## Role

Explicit query-access descriptor law.

## Data Model

Access descriptors declare read, write, publication, scratch, and partitionability rights for compiled query plans.

## Descriptor Matrix

| Descriptor field | Canonical law |
|---|---|
| read/write rights | explicit and typed |
| publication rights | explicit when present |
| scratch class | explicit and bind-time checked |
| hidden widening | illegal |

## Canonical Law

- access rights must be explicit, typed, and checked at traversal bind;
- hidden write widening through helper wrappers is illegal;
- access-descriptor shape changes are legal invalidation triggers for traversal plans.
