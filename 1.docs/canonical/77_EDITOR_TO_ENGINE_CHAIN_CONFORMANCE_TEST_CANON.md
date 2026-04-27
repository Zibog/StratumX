# Editor To Engine Chain Conformance Test Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the live conformance test for the downward command spine.

## Assertions
1. every promoted concrete button in `editor/110` has one owner lab;
2. every promoted concrete button has one tooling route id;
3. every tooling route id resolves to one sdk packet family;
4. every sdk packet family resolves to one engine truth owner set;
5. every promoted row declares one allowed lifecycle from sdk `77`;
6. every compare/capture/recover/certify/freeze row returns a traceable `artifact_ref` on terminal success;
7. every denial path returns one `ack.denied` or one accepted-path failure state with explicit failure code;
8. every success path returns one success code, one focus target, and one next legal recovery or comparison step;
9. wildcard or family-only rows do not count as conformance evidence.

## Required live evidence
- `editor/110` promoted-row snapshot;
- route manifest snapshot from tooling;
- sdk packet-family snapshot;
- engine owner crosswalk;
- runtime trace sample for every promoted class: author, bind, inspect, simulate, compare, capture, recover, certify, review, signoff;
- lifecycle sample showing at least one success path, one retryable failure, one terminal failure, and one deny-before-execution path.

## Failure posture
Any missing link marks the chain incomplete.
Any command that executes without visible trace, lifecycle state, focus target, or next legal action fails conformance.

## Phase linkage
This test is mandatory for phase 1 exit.
Until it is measured live, the archive remains `doc_closed_impl_open`.
