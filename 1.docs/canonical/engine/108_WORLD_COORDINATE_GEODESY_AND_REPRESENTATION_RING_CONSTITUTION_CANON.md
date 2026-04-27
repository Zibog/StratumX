# World Coordinate Geodesy And Representation Ring Constitution Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the canonical coordinate split for a 1:1-scale world.

## Coordinate classes
| Class | Use | Precision rule |
|---|---|---|
| `coord.world` | region placement, climate theater, far observability | coarse-stable |
| `coord.region` | streaming anchor and save/restore continuity | stable |
| `coord.cell` | local simulation partitions and field storage | high |
| `coord.local` | contact, animation, projectile, wound traversal | highest |

## Rebasing law
Rebasing may shift region/cell anchors.
It may not change retained world identity, climate-front identity, or proof-region references.

## Representation rings
`ring.live_exact`, `ring.reduced_exact`, `ring.statistical_far`, `ring.retained_summary` are the only legal far-world classes.
No hidden ad-hoc impostor class may bypass them.
