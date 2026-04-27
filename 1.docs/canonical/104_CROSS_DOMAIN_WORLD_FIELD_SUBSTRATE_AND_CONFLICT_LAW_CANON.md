# Cross Domain World Field Substrate And Conflict Law Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define one canonical field substrate for wetness, heat, smoke density, contamination, wind, visibility obscuration, sound pressure hints, and anomaly fields.
The world may contain many field families; it may not contain many unrelated field systems pretending to be one world.

## Field substrate families
| Field family | Legal scopes | Primary consumers |
|---|---|---|
| wetness | cell, surface, object-local | fire, terrain, cloth/fur, footstep/audio, shading |
| heat | cell, surface, volume | fire, hydrology, damage, VFX, lighting hints |
| smoke density | cell, volume | visibility, lighting, health/wound, audio, VFX |
| toxic contamination | cell, surface, volume | living health, AI avoidance, weather transport |
| wind vector | region, cell, volume | fire spread, smoke, cloth/fur, projectiles, weather |
| visibility obscuration | cell, volume | rendering, tactics, perception, certification |
| sound pressure hint | cell, zone, surface | audio mixing, tactics, stealth heuristics |
| anomaly field | cell, surface, volume | traversal, wounding, AI risk, VFX |

## Storage law
- **cell field**: tiled, streaming-safe, authoritative for world-scale propagation;
- **surface field**: surface-linked modulation for terrain/material and aftermath;
- **object-local field**: local container or host-bound truth such as heat in a barrel or wetness in a garment;
- **volume field**: bounded 3D carrier for smoke, gas, or anomaly media.

## Update and conflict order
1. ingest external drivers from weather, hydrology, destruction, living, and devices;
2. advance carrier fields by substrate family;
3. resolve competing writes by family precedence: source truth -> substrate carrier -> derived presentation;
4. publish changed field bands and affected heavy-domain consequences;
5. seal replay digest.

## Conflict law
- field conflict is resolved by canonical precedence, never by last-write-wins convenience;
- object-local overrides are legal only inside declared host bounds;
- surface-field overrides may modulate but may not hide cell-field carriers;
- presentation-only fields are forbidden to feed back into authoritative world truth.

## Scope/tier law
Every field write must publish:
- field family id;
- scope class;
- simulation tier;
- retention horizon;
- degradation posture when cadence or resolution drops.

## Compare and proof duties
Field-driven proof bundles must retain both:
- carrier-band digests;
- consequence digests on at least one consuming domain.

## Current posture
`document_gold / substrate_closed / runtime_impl_open`
