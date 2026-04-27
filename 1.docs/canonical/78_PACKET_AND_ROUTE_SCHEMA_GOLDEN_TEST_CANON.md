# Packet And Route Schema Golden Test Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the mandatory golden test for route and packet schema completeness.

## Assertions
1. every promoted command packet family declares field tables and version ids;
2. every promoted observation packet family declares field tables and version ids;
3. compare, capture, recover, certify, freeze, and signoff packet families declare retained-artifact duties;
4. every route declares transaction states, retry limits, rollback anchors, invalidation triggers, and compatibility rules;
5. lifecycle-carrying payloads declare which sdk `77` states they may emit;
6. every failure-bearing payload declares failure code family and next legal action fields;
7. no unnamed payload, unnamed artifact, or unversioned replay-bearing message is allowed.

## Required live evidence
- sdk field-table export for command and observation packets;
- route-state export including retry budgets and rollback anchors;
- compatibility rule extract for replay-bearing and compare-bearing payloads;
- artifact-ownership manifest;
- lifecycle-emission matrix;
- denied/success/failure payload samples.

## Failure posture
Any family without named fields, codes, states, lifecycle declaration, or artifact duties fails the golden test.

## Phase linkage
This test is mandatory for phase 1 exit and is reused by phases 2–6.
