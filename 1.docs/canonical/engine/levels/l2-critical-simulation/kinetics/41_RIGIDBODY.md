# Rigidbody

## Role

Bounded rigidbody solve inside `engine_kinetics`.

## Rigidbody Matrix

| Concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| island scope | bounded island solve only | defer low-value islands first | global monolithic solve |
| contact persistence | bounded cache only | evict low-value contact history first | unbounded persistence cache |
| mutation handoff | staged through world/apply | drop optional side records first | direct world mutation |
| bridge records | bounded by cross-family law | collapse excess bridge records | unlimited same-tick coupling |

## Rule

Rigidbody solve may not widen contact caches, island scope, or world mutation rights beyond the bounded classes frozen by kinetics law.

## Operating Matrix

| Operating concern | Preserve first | Degrade first |
|---|---|---|
| near island legality | authoritative rigidbody stability | low-value islands |
| contact-history pressure | bounded contact continuity | optional old contacts |
| bridge pressure | bounded same-tick legality | excess cross-family aftermath |

## Local Operating Law

Rigidbody is a bounded local operating law, not a generic `physics` blob.
It may not acquire global-solve entitlement or direct world-mutation rights.
