# World Chunk Invalidation And Rebuild Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the law for chunk invalidation, rebuild, load/save eligibility, and stale-state denial.

## Core law
Any authoring or runtime change that affects terrain topology, material layer weights, biome overlays, aftermath overlays, placements, or environment-derived chunk data must publish chunk invalidation explicitly.

## Mandatory chunk states
- clean;
- dirty-authoring;
- invalidated-awaiting-rebuild;
- rebuilt-not-saved;
- loaded-stale-against-manifest;
- restored-from-baseline;
- denied-illegal.

## Rebuild law
A rebuild must publish:
- affected chunk ids;
- cause family;
- produced revision ids;
- whether save is now legal or still blocked.

No world save may persist chunks that remain in `invalidated-awaiting-rebuild` or `denied-illegal`.
