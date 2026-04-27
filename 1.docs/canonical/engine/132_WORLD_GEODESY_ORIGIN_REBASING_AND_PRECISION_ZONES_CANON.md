# World Geodesy Origin Rebasing And Precision Zones Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own exact coordinate law for a 1:1-scale world, including geodesy, region/cell frames, origin rebasing, and precision zones.

## Exact truth objects
- `GeoAnchorLedger`
- `RegionFrameSet`
- `CellFrameSet`
- `RebaseDeltaLedger`
- `PrecisionZoneContract`
- `FarPhenomenonTrackSet`

## Exact phase order
1. resolve geodetic anchor and region frame;
2. derive active cell frames and precision zone;
3. publish or apply rebase deltas for local simulation lanes;
4. map far-phenomenon tracks into observable theater projections;
5. seal stable replay digest.

## Precision-zone law
| Zone | Typical radius | Mandatory exactness |
|---|---:|---|
| `pz.contact` | `0–5 m` | local exact floats, contact-safe wound/cloth/animation solve |
| `pz.interaction` | `5–500 m` | exact world truth for gameplay, fields, and ballistics |
| `pz.extended` | `500 m–5 km` | exact or reduced-exact by ring contract, never hidden identity drift |
| `pz.theater` | `5–200 km` | summarized far-causality carriers only |

## Rebase law
- rebasing may not change ids, chronology, or world package truth;
- actor-local transforms must know which rebase delta they were resolved against;
- far tracks are region-relative and survive many rebases without accumulating local noise.

## Failure families
- `world.geo.anchor_missing`
- `world.geo.rebase_unpublished`
- `world.geo.precision_zone_illegal`
- `world.geo.far_track_drift`

## Proof duties
Certification captures must retain geodetic anchor id, region frame id, active precision zone code, and rebase delta lineage.

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
