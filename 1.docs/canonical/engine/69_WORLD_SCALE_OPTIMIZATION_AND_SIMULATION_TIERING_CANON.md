# World Scale Optimization And Simulation Tiering Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Own hot/warm/cold simulation tiers, promotion/demotion rules, and legal optimization boundaries for world-scale workloads.

## Exact truth objects
- `simulation_tier_state`
- `promotion_queue_state`
- `demotion_queue_state`
- `domain_budget_band`
- `active_bubble_contract`
- `bubble_ring_contract`
- `tier_reason_digest`

## Exact state machine
`measure -> score -> promote / retain / demote -> publish -> recover`

## Exact phase order
1. measure current domain pressures;
2. score entities, cells, and families against active-bubble contract;
3. apply bounded promotion/demotion changes;
4. publish tier reason digests and ladder state;
5. verify recovery before next climb.

## Coupling boundaries
| Boundary role | Declared links | Forbidden shortcut |
|---|---|---|
| reads | root `44` budget orchestra; engine `60` streaming residency; engine `98` cross-technology arbitration; engine `103` backend feature posture | never owns domain truth semantics |
| publishes | engine `60` residency bands; sdk hardware-floor packets; editor `74`, `81`, `89` pressure views | may not hide degrade ladder steps from operators |

## Ring-aware tiering law
Tiering must be aware of bubble rings, not just raw distance.
The score for promotion / retention / demotion must be allowed to combine:
- distance ring;
- recent interaction;
- visibility importance;
- tactical or traversal relevance;
- acoustic relevance;
- persistent consequence relevance;
- hardware-floor pressure.

An object in the outer exact ring may retain exact truth while carrying reduced richness.
That is legal and preferred over shrinking the truthful world too early.

## Resource envelope
- CPU: green <= `0.8 ms` arbitration, yellow <= `1.2 ms`, orange <= `1.8 ms`, red > `1.8 ms`;
- GPU: none directly; may request degrade on GPU red but never fake success;
- RAM: orange > `256 MiB` tier ledgers, red > `320 MiB`;
- disk / IO: only artifact-backed tier traces; blocking write forbidden.

## Ordered degrade ladder
- reduce far-family cadence in declared order;
- reduce outer-exact richness before shrinking exact-truth legality;
- compact tier reason history;
- sample non-critical overlays;
- defer secondary artifacts after baseline retention.

## Replay and compare windows
- `baseline.300`;
- `cert.600`;
- `restore.300`;
- `oldfloor.2400`;

## Failure and denial code families
- `tier.promote.illegal`
- `tier.demote.hidden`
- `tier.recovery.skip`
- `tier.reason.missing`
- `tier.ring.contract_missing`

## Certification duties
- retain one full ladder trace for every combined old-floor pack;
- publish explicit reason for every promote/demote step;
- emit denial if a requested profile would skip declared tier order;
- expose region-scale validation posture to `editor/89`;
- expose ring-level truth vs richness posture to `editor/74` and `editor/81`.

## Current posture
`document_gold / doc_closed_impl_open`

---

# V34 world/entity/scheduler implementation decisions

Stack version: `SX-CANON/1.0.28/STACK-v34`

## World hierarchy

`World → Region → Sector → Cell → Chunk → Entity/Surface/Field`.

Authoring coordinates may use stable high precision global positions. Runtime hot simulation uses cell-local coordinates. Streaming loads chunks; regions are packaging/authoring scopes.

## Entity model

`WorldEntity` is stable identity. Components are data. Systems own behavior. Editor object is authoring projection, not runtime truth.

## Scheduler model

Engine owns the phase scheduler. Domains publish jobs into phases. Domains do not spawn arbitrary global threads.

Required phases:

1. Input
2. AuthoringCommands
3. WorldMutation
4. Simulation
5. Physics
6. AI
7. Animation
8. AudioPrepare
9. RenderExtract
10. RenderSubmit
11. Diagnostics
12. Persistence

## Physics strategy

StratumX may use a replaceable physics kernel adapter for collision/motion. StratumX truth remains above it in material response, topology, field substrate, aftermath, diagnostics, and persistence.
