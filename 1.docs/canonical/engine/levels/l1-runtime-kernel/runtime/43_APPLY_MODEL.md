# Apply Model

## Role

Runtime apply publication model.

## Data Model

Apply consumes staged records from legal writers and publishes authoritative world changes through segmented apply only.
Apply is bounded by segment count, segment fan-out, and publication order law.

## Apply Matrix

| Item | Canonical law |
|---|---|
| apply ownership | runtime/world only |
| segment count | bounded by apply law |
| publication order | deterministic inside runtime phase law |
| direct mutation outside apply | illegal |

## Illegal Patterns

- world mutation bypassing apply boundary;
- unbounded micro-segmentation to hide spikes;
- family-owned apply ordering.
