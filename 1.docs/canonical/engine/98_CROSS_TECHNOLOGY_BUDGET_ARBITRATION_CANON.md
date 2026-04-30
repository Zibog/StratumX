# Cross-Technology Budget Arbitration Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze the engine-side runtime truth doctrine for cross-technology budget arbitration.
It belongs in `engine/` because the engine owns the final runtime consequence.
Upper layers may request, route, observe, compare, or visualize this domain.
Upper layers may not redefine its runtime truth.

## Truth objects
- `budget_vector_state`
- `pressure_signal_set`
- `recovery_state`

## Runtime phases
1. sample budgets;
2. arbitrate pressure;
3. apply degradation or recovery;
4. publish budget state.

## Ingress and egress carriers
- ingress: domain pressure publications only;
- egress: typed budget observations, typed denial publications, and typed recovery posture.

## Diagnostics and failure classes
- `ARBITRATION_CONFLICT`
- `RECOVERY_STALL`

## Degradation and fallback law
Every domain in this file must define:
- what degrades first under pressure;
- what may not degrade without explicit verdict;
- which fallback still preserves truthful operator understanding; and
- which fallback becomes illegal because it would fake a result.

## Forbidden shortcuts
- editor may not bypass engine truth by presenting local approximations as runtime truth;
- tooling may not suppress engine denial codes;
- sdk may not normalize away domain-critical distinctions;
- a successful observation may not be published if the runtime phase failed and no legal fallback exists.

## Evidence obligations
Promotion beyond `doc_closed_impl_open` requires:
- at least one runtime path test;
- one diagnostics capture proving the phase order;
- one observation sample proving the carrier schema; and
- one failure sample proving denial publication.

## Phase-6 obligations
- every mixed old-floor result must preserve the actual arbitration rung per participating family;
- every region-scale validation review must expose the last blocking arbitration state;
- freeze review may not accept a summary that omits the first blocking arbitration verdict.

## Current posture
`doc_closed_impl_open` unless explicitly upgraded by the implementation reality ledger.

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
