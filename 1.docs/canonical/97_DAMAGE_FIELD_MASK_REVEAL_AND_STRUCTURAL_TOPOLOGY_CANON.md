# Damage Field Mask Reveal And Structural Topology Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the cheap-but-honest runtime contour for cracks, holes, splinters, charring, wet spread, and structural weakening without requiring arbitrary mesh surgery.

This document exists so impact and destruction are no longer forced into the false choice between:
- fake decals with no truth;
- or full geometry fracture for everything.

## Damage-field truth law
A damage field is a bounded local truth carrier attached to one active material package or one active terrain patch.
It owns compact state such as:
- integrity drop;
- crack level;
- breach level;
- char level;
- wetness reveal;
- release-group eligibility;
- support-group weakening.

It is not a replacement for world identity, support graphs, or global geometry ownership.

## Canonical field resolutions
The runtime must normalize the following field families.

| Family | Typical scope | Cheapness meaning |
|---|---|---|
| `field.8x8.local` | very small props or cheap previews | point event, low-memory |
| `field.16x16.local` | crates, doors, table planks, medium props | near-object cheap truth |
| `field.32x32.surface` | walls, slabs, bigger panels, hero props | standard active patch |
| `field.64x64.hero` | large close-range authoring / proof slice only | bounded high-detail local truth |
| `field.cell_patch.coarse` | terrain or structure support tiles | occupancy/support carrier, not cosmetic detail |

## Canonical mutation phases
`classify -> stamp_primary_damage -> propagate_topology -> evaluate_breach_or_release -> publish_support_and_occupancy -> summarize`

## Trigger-shape families
Damage-field mutation must first classify trigger shape.

| Trigger-shape family | Meaning |
|---|---|
| `shape.point` | bullet, tool strike, sharp puncture |
| `shape.line` | slash, scrape, grazing shear |
| `shape.cone` | directed blast or fragment spray |
| `shape.area` | radial blast, collapse slap, heavy contact patch |
| `shape.edge_erosion` | burn-through, dissolve, long-duration weathering |
| `shape.field_spread` | wet spread, soot spread, ash spread |

## Operator families
A damage field may only mutate through canonical operator families owned by the material package.
The baseline operator families are:
- `op.point.chip`
- `op.point.punch`
- `op.point.radial_crack`
- `op.line.graze_shear`
- `op.area.spall`
- `op.area.breach`
- `op.area.collapse_edge`
- `op.edge.burn_erosion`
- `op.field.char_progression`
- `op.field.wet_spread`
- `op.edge.splinter_treatment`

## Seed law
Operators may be pseudo-random but not arbitrary.
A lawful seed must derive from:
- event seed;
- material package id;
- support-group context;
- local topology digest;
- optional module/joint index when the assembly pattern is repeated.

This gives variation without losing repeatability, debugability, or compare friendliness.

## Structural-topology law
A damage field must expose the minimum structural-topology carriers needed for lawful downstream consequence.

| Carrier | Meaning |
|---|---|
| `support_group_id` | which local support group or bearing patch is weakened |
| `release_group_id` | which local piece or module may detach |
| `occupancy_tile_digest` | coarse open / blocked / weakened digest for collision and traversal |
| `breach_digest` | whether a legal hole exists and through which layer |
| `exposed_inner_face_digest` | which cross-section profile is now visible |

A route is illegal if it shows a hole but publishes no lawful breach digest.

## Patterned-assembly law
For `assembly.masonry_grid`, `assembly.plank_array`, and other repeated assemblies, operator propagation must be allowed to bias toward:
- seams;
- joints;
- module edges;
- repeated support spacing.

A brick wall is therefore allowed to crack or release along joint-biased topology without needing per-brick full rigid-body simulation for the whole structure.

## Terrain and landscape law
Landscape damage is legal through cell patches and terrain surface fields.
It must not require per-triangle fracture.
A lawful landscape route may publish:
- crater patch;
- rut patch;
- churned mud patch;
- burn patch;
- vegetation-clear patch;
- support-cost change for traversal.

## Collision and navigation publication law
When damage becomes world-relevant, downstream consumers must read coarse digests, not arbitrary render meshes.
Collision, navigation, cover, and traversal may consume:
- occupancy tiles;
- support state;
- breach digest;
- residue / rubble digest.

They may not depend on the renderer’s local alpha reveal as authority.

## Memory law
- damage fields are allocated only for awake or recently active material packages;
- dormant instances must prefer summary digests;
- far-world state must prefer retained aftermath summaries;
- `field.64x64.hero` is legal only inside bounded proof slices or near-camera hero cases.

## Forbidden shortcuts
- one giant texture mask stored permanently for every world object;
- random crack masks with no repeatable seed law;
- visual-only holes with no occupancy or breach digest;
- per-triangle rebuild as the default path for ordinary impact.

## Current posture
`document_gold / closes_damage_field_and_mask_reveal_gap`
