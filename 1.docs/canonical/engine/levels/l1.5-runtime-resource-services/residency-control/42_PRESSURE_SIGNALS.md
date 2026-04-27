# Pressure Signals

## Role

Pressure and eviction visibility.

## Data Model

Pressure is explicit and service-owned.
Signals must name the breached profile envelope, the breached allocator or residency class, and the applied mitigation step.

## Canonical Ladder

1. trim optional warm caches;
2. deny optional prefetch;
3. delay optional uploads;
4. evict cold sets;
5. demote warm sets;
6. fail illegal requests.

Silent pressure absorption is illegal.
