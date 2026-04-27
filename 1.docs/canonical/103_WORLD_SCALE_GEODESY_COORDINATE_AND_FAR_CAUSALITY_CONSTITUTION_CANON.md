# World Scale Geodesy Coordinate And Far Causality Constitution Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the exact coordinate, precision, rebasing, and far-causality law for a 1:1-scale world.
This closes the gap between “large world exists” and “large world is mathematically and operationally lawful”.

## Canonical space split
| Space | Owner | Typical extent | Precision duty | Forbidden shortcut |
|---|---|---:|---|---|
| geodetic world space | world/engine geodesy spine | planet / theater scale | stable coarse identity and absolute location | may not be fed directly into fine local simulation |
| region space | engine world partition | `10–80 km` regions | stable streaming and climate partition anchors | may not hold high-frequency local contact truth |
| cell space | engine runtime bubble | `64–512 m` cells | exact residency, fields, and most heavy-domain truth | may not drift independently of region anchor |
| actor-local space | object/living host runtime | model / rig / local constraint scale | exact contact, wound, cloth, animation, and fine deformation | may not become a hidden world-space substitute |
| far-phenomenon track space | climate / fire / migration fronts | `5–200 km` observability | stable summarization and determinism for far-visible events | may not pretend to carry exact near-object truth |

## Precision contracts
| Horizon band | Required contract |
|---|---|
| `0–5 m` | full local precision for contact, wound traversal, small deformation, muzzle flash, audio timing |
| `5–500 m` | exact interactive truth, lawful culling, deterministic short-horizon replay |
| `500 m–5 km` | exact or reduced-exact truth by ring contract, stable causes must survive degradation |
| `5–30 km` | retained summary / far simulation truth only; player-visible consequences must remain attributable |
| `30–200 km` | theater-scale climate, storm, fire-front, migration-front, skyline, and light observability only |

## Rebase law
- rebasing is a view/runtime convenience, never a change to canonical identity;
- region anchors are stable across save/restore;
- cell anchors may move under rebasing but must publish the delta;
- actor-local truth may never observe a silent origin jump;
- far-phenomenon tracks survive rebasing through region-relative anchors, not through repeated quantization loss.

## Far-causality law
The archive distinguishes:
- **near exact causality**: exact local objects and fields change each other directly;
- **far causal carriers**: fronts, storm cells, migration waves, contamination plumes, distant fires, and light/weather observability advance through theater-scale summaries;
- **observed consequence contract**: when the player may lawfully observe a distant phenomenon, one retained carrier must explain what was seen and how it later arrived.

## Required publications
- `geo_anchor_ref`
- `region_frame_ref`
- `cell_frame_ref`
- `rebase_delta_ref`
- `far_phenomenon_track_ref`
- `precision_zone_code`

## Degrade and certification law
Old-floor profiles may degrade update cadence and spatial richness, but they may not degrade identity stability, rebase publication, or far-causality explainability.

## Bound companions
- engine `108`, `114`, and `115` own the heavy-domain runtime specifics.
- sdk `79–82` own packet families and replay surfaces.
- editor `122`, `126`, and `127` own operator and proof routes.

## Current posture
`document_gold / deep_spec_closed / runtime_impl_open`
