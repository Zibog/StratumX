# Hydrology Containers Leaks And Evaporation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define runtime truth for container fill state, leak topology, hole-level outflow, pooling, evaporation, restore anchors, and compare-safe replay.

## Exact truth objects
| Truth object | Meaning |
|---|---|
| `ContainerMassLedger` | authoritative per-container mass and fill delta |
| `LeakTopologyGraph` | active leak edges, topology lineage, and rupture class |
| `PoolDepthField` | surface pool bands and containment boundaries |
| `EvaporationTierField` | quantized evaporation tiers and loss coefficients |
| `RestoreAnchorSet` | last-good restore anchors for replay and certification |
| `HydroReplayWindow` | replay horizon id, triplet binding, and compare identity |

## Exact state machine
`stable -> filling / draining -> leaking -> pooling -> level_locked -> evaporating -> restored / failed`

## Exact phase order
1. resolve source, sink, and hole-level legality.
2. integrate mass delta and publish conserved mass ledger.
3. update leak topology and outflow stop condition.
4. settle pool and evaporation tiers.
5. publish diagnostics, replay digest, and recovery anchors.

## Exact coupling boundaries
- may consume weather precipitation from engine `62` and wetness requests from engine `49` as read-only inputs.
- may not mutate persistence truth directly.
- must expose restore-anchor legality before replay consumers may act.
- must publish whether outflow stopped because the fluid level reached the aperture height.

## Exact compare / capture / certification law
| Action family | Allowed ids | Pack |
|---|---|---|
| compare | `compare.hyd.restore_triplet`, `compare.timeline.restore_triplet` | `pack.hydrology_persistence` |
| capture | `capture.hyd.ledger_bundle`, `capture.timeline.restore_bundle` | `pack.hydrology_persistence` |
| recover | `recover.hydrology.*`, `recover.timeline.*` | `pack.hydrology_persistence` |
| certify | phase-2 certification through editor `66` only | `pack.hydrology_persistence` |

## Phase-2 brutal proof slice
`rainfall fill -> shot leak -> outflow until hole level -> leak stop -> evaporation over time -> restore/replay compare`

## Exact fail / denial families
- `hydro.mass.nonconserve`
- `hydro.restore.anchor_invalid`
- `hydro.leak.topology_missing`
- `hydro.level_lock_unbound`
- `hydro.evaporation.tier_unbound`

## Exact resource envelope
cpu `<= 2.8 ms`; gpu overlays `<= 0.6 ms`; ram `<= 512 MiB`; disk/IO restore bundles only

## Current posture
`document_gold / doc_closed_impl_open`
