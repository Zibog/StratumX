# Resource Pressure Operator Visible Law Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define one ruthless, operator-visible pressure vocabulary for heavy-domain certification on the declared old-hardware floor.
This file is not presentation garnish.
It is the visibility contract that all heavy packs, routes, labs, and freeze reviews must obey.

## Four mandatory axes
Every pressure-bearing route, certification pack, compare run, failed run, recovery run, and freeze review must publish:
- active axis;
- current bucket;
- active ladder step;
- threshold row id;
- compare horizon id;
- retained baseline artifact id;
- first blocking code;
- next legal recovery action.

## Exact threshold tables
| Axis | green | yellow | orange | red | hard fail |
|---|---|---|---|---|---|
| CPU frame time | `<= 12.5 ms` | `<= 14.0 ms` | `<= 15.5 ms` | `<= 16.6 ms` | `> 16.6 ms sustained for 120 frames` |
| GPU frame time | `<= 10.5 ms` | `<= 12.0 ms` | `<= 14.0 ms` | `<= 16.6 ms` | `> 16.6 ms sustained for 120 frames` |
| RAM residency | `<= 4.5 GiB` | `<= 5.0 GiB` | `<= 5.5 GiB` | `<= 6.0 GiB` | `> 6.0 GiB or eviction thrash` |
| disk / streaming I/O hitch | `<= 4 ms` | `<= 8 ms` | `<= 12 ms` | `<= 16 ms` | `> 16 ms or queue starvation` |

## Exact domain threshold tables
| Domain | CPU ceiling | GPU ceiling | RAM ceiling | disk / IO ceiling | compare horizon | first degrade step | first legal recovery step |
|---|---|---|---|---|---|---|---|
| large world / streaming | `<= 4.5 ms` | `<= 2.1 ms` | `<= 4.0 GiB` | `<= 12 ms` | `1800 frames` | shrink far residency radius | restore one residency ring after one full green horizon |
| hydrology / persistence | `<= 2.8 ms` | `overlay-only <= 0.8 ms` | `<= 512 MiB` | `<= 10 ms restore hitch` | `1200 frames + restore` | reduce far-field update cadence | restore one cadence tier after stable mass-balance compare |
| fire / weather / smoke | `<= 3.2 ms` | `<= 2.8 ms` | `<= 384 MiB` | `<= 8 ms` | `900 frames` | lower volumetric march density | restore one volumetric rung after green visibility compare |
| cloth / fur / soft surface | `<= 3.4 ms` | `overlay-only <= 1.3 ms` | `<= 320 MiB` | `frame-invisible only` | `900 frames` | reduce solver density one rung | restore one solver rung after stable contact compare |
| population / tactics | `<= 8.3 ms combined` | `overlay-only <= 1.0 ms` | `<= 640 MiB` | `<= 8 ms` | `1200 frames` | reduce far-agent decision cadence | restore one decision tier after legal suppression and flank compare |
| ecology / migration | `<= 3.2 ms` | `overlay-only <= 0.7 ms` | `<= 384 MiB` | `<= 8 ms` | `1200 frames` | reduce distant migration solve cadence | restore one route tier after stable migration compare |
| wounds / ballistics | `<= 4.6 ms combined` | `overlay-only <= 0.8 ms` | `<= 320 MiB` | `<= 8 ms` | `900 frames` | compact history payloads | restore one history tier after replayable lethality compare |
| semantic runtime | `<= 4.4 ms combined` | `n/a` | `<= 448 MiB` | `<= 10 ms` | `dialogue chain + restore` | compact explain payload size | restore one explain tier after policy-stable consequence compare |

## Ordered degrade ladders
| Axis | Ordered degrade ladder |
|---|---|
| CPU | lower broadphase or route density -> lower expensive agent update cadence -> reduce far-field heavy-domain frequency -> sample non-authoritative overlays |
| GPU | lower volumetric step count -> reduce shadow or translucency refresh -> reduce distant detail -> clamp post stack cost |
| RAM | shrink far residency radius -> drop non-critical compare caches -> compact replay slices to declared window -> evict non-active artifact mirrors |
| disk / I/O | widen prefetch lead -> reduce non-critical capture frequency -> serialize artifact writes after frame-critical reads -> step down background verification passes |

## Recovery law
- recovery always proceeds in exact reverse order of the active degrade ladder;
- no domain may jump from sampled or clamped mode directly to maximal mode;
- a full green compare horizon is required before every upward recovery step;
- the operator-visible recovery decision must match the domain row in this file or freeze remains blocked.

## Last-good baseline obligations
- every red or orange run must expose the last-good baseline id;
- failed runs may never overwrite the last-good baseline;
- recovery runs must point to the exact failed-run id they recover from;
- if baseline identity is missing, freeze is blocked immediately.
