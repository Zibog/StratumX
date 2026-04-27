# Scene Extraction

## Role

Authoritative-state to render-state extraction.

## Data Model

Scene extraction consumes snapshot-safe authoritative data and emits bounded imaging descriptors for the current frame.
Extraction owns no world truth and may not widen authority or visibility ownership.

## Canonical Law

- extraction uses snapshot-safe data only;
- extraction cost is bounded by absolute budgets and crate performance budgets;
- extraction may not pull optional IO, decode, or verification work into the frame path.

## Illegal Patterns

- direct world mutation from extraction;
- extraction-owned residency or transfer law;
- using stale visibility beyond legal staleness ceilings.
