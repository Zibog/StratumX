# Physical Data Layout Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

L5 uses three physical data planes:
- immutable bridge snapshots
- ordered ingress lanes
- immutable egress batches

## Immutable bridge snapshots
Bridge snapshots contain:
- compatibility facts
- compatibility verdict tables
- transport policy tables
- legality lookup tables
- handle tables
- ref tables
- artifact-ref tables

Bridge snapshots are low-churn and replaceable by declared bridge epochs.
They must remain immutable once published.

## Ordered ingress lanes
Ingress lanes contain:
- packet headers
- control headers
- payload refs
- order keys
- source and target handles

Ingress lanes are write-side only.
They must remain narrow, ordered, and free of editor semantics.

## Immutable egress batches
Egress batches contain:
- observation headers
- metric headers
- payload refs
- batch anchors
- optional ref deltas
- monotonic cursor state

Egress batches are read-side only.
They must be fanout-safe and immutable once published.

## Layout law
Hot headers and routing keys must remain locality-oriented and bounded.
Cold diagnostics, verbose reason strings, and editor-only metadata must remain outside hot publication layouts.
