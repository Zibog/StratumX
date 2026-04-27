# Lifetime Model

## Role

Allocation lifetime law.

## Data Model

Allocator classes are lifetime-partitioned and may not borrow legality from one another.

## Canonical Lifetime Rules

- `frame-scratch` dies at frame end and may not cross into next frame;
- `tick-scratch` dies before the next authoritative tick and may not cross apply boundary;
- `cell-scratch` dies when local region/chunk work closes;
- `session-persistent` survives until session shutdown or explicit persistent-cache trim;
- `streaming-resident` survives by residency law only;
- `staging-backed` survives only until fence-visible release.
