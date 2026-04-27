# World Persistence And Save Restore Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze how world state is persisted, restored, replayed, and compared without losing identity or chunk/material/environment coherence.

## Save law
A lawful world save must capture at minimum:
- world identity;
- chunk revisions or delta lineage;
- material linkage state or references;
- environment binding state;
- placement and persistent aftermath state;
- validation posture and restore anchors when freeze-relevant.

## Restore law
A restore is lawful only if the operator can prove:
- the saved state resolves to one world identity;
- chunk revisions or deltas can be replayed without ambiguity;
- material and environment bindings still resolve;
- degraded posture, if present, is declared rather than silently changed.

## Compare law
World persistence proof must retain baseline, failed-run, and recovered-run identities when the world route participates in certification or release.
