# Interest Model

## Role

Authoritative sync relevance model.

## Data Model

Interest is region-first, role-aware, tier-aware, and budget-bounded.
Per-connection export may not ignore region, tier, and visibility relevance.

## Canonical Ordering

1. region and chunk relevance;
2. tier and authority relevance;
3. role or visibility relevance;
4. per-connection byte budget.

A connection may not bypass region-first relevance just because an entity looks important in isolation.
