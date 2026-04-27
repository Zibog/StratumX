# Pressure Response

## Role

Allocator pressure response.

## Data Model

Numeric Authority: STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md §6.2


Pressure response is budget-driven and ordered.
Allocator pressure may not silently spill into unrelated classes.
Mandatory response latency is frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Canonical Response Ladder

1. trim optional persistent caches;
2. reuse or compact pool-owned slabs;
3. deny optional prefetch or optional upload staging;
4. escalate to residency demotion or eviction;
5. fail the request if the profile ceiling would be crossed.

## Mandatory Response Latency

A class-ceiling breach must enter the canonical response ladder within <= 2 realtime frames or <= 2 headless ticks.

## Illegal Patterns

- hidden fallback to general-purpose heap in a critical execution lane;
- moving `tick-scratch` debt into `session-persistent` without explicit canon rule;
- fence-bound staging memory retained past release visibility.
