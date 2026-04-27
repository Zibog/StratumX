# Large World Streaming Residency And Long Range Visibility Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own region residency, world-cell activation, far-field visibility legality, and long-range truth publication without fake continuity.

## Exact truth objects
- `world_region_state`
- `residency_band_state`
- `long_range_visibility_band`
- `stream_request_window`
- `cell_history_anchor`
- `active_bubble_boundary`
- `bubble_ring_contract`

## Exact state machine
`unmapped -> mapped_cold -> warming -> resident -> active -> cooling -> cold_persisted -> published`

## Exact phase order
1. compute active bubble and region priority sets;
2. issue bounded stream requests;
3. promote legal residents into simulation tiers;
4. publish far-field visibility and residency reasons;
5. retire cooled cells with persistence anchors.

## Coupling boundaries
| Boundary role | Declared links | Forbidden shortcut |
|---|---|---|
| reads | 69 tiering policy; 87 visibility query budgets; 102 IO pressure and warmup policy; 103 backend feature posture | never owns content import or editor streaming controls |
| publishes | 46/57 sdk streaming and hardware-floor packets; 73 persistence anchors | may not invent geometry or quest truth for unloaded cells |

## Bubble-ring law
The active bubble may expose multiple exactness rings.
A lawful large-world contour is allowed to keep roughly 300 m of exact-truth coverage around the player or other primary operator focus, provided richness is ring-bounded.

| Ring | Typical distance | Lawful posture |
|---|---|---|
| `ring.core` | `0–40 m` | highest interaction richness, full local truth for recently interacted and high-risk objects |
| `ring.near_exact` | `40–120 m` | exact truth with reduced fragment richness and reduced cosmetic patch density |
| `ring.outer_exact` | `120–300 m` | exact truth only for important or activated objects; reduced update cadence and summary-friendly representation |
| `ring.cached_local` | outside outer exact to local horizon | cached local summaries, HLOD, reduced world mutation cadence |
| `ring.far_summary` | long-range bands | retained state digests, visibility truth, long-range presentation, no fake local detail |

Exact truth at 300 m is lawful.
Exact richness for everything at 300 m is not required and may be illegal on old-floor profiles.

## Resource envelope
- CPU: green <= 1.8 ms, yellow <= 3.0 ms, orange <= 4.5 ms, red > 4.5 ms.
- GPU: visibility contribution orange > 1.5 ms from far-field only, red > 2.1 ms.
- RAM: green <= 2.4 GiB world residency, orange > 3.4 GiB, red > 4.0 GiB.
- disk / IO: orange when read burst > 24 MiB/frame-equivalent, red when hitch > 12 ms.

## Ordered degrade ladder
- reduce distant update cadence;
- reduce far visibility precision bands before shrinking active bubble;
- reduce outer-exact richness before shrinking exact-truth legality;
- compact cold-cell history mirrors;
- defer non-critical capture artifacts until green recovery.

## Replay and compare windows
- `baseline.600`;
- `cert.1200`;
- `restore.600`;
- `travel.2400`;

## Failure and denial code families
- `world.stream.residency_gap`
- `world.visibility.band_fraud`
- `world.cell.identity_drift`
- `world.io.hitch_red`
- `world.bubble.ring_hidden`

## Certification duties
- retain one residency ladder trace for every combined old-floor pack;
- publish stable cell identity digest across restore runs;
- emit denial when an operator request would force illegal residency promotion;
- publish the active bubble ring contract alongside old-floor captures.

## Current posture
`document_gold / doc_closed_impl_open`
