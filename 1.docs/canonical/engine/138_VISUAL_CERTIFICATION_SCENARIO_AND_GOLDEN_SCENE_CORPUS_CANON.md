# Visual Certification Scenario And Golden Scene Corpus Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Name the golden scenes used by visual certification and comparison.

## Golden corpus rows
| Corpus id | Scene | Required captures |
|---|---|---|
| `gold.visual.tunnel.v1` | tunnel firefight | frame, light tier report, audio continuity report, first blocked feature verdict |
| `gold.visual.storm.v1` | distant storm horizon | horizon frame, weather front timeline, observability verdict |
| `gold.visual.wet_forest.v1` | wet forest dusk | foliage shadow report, wetness readability frame, rung digest |
| `gold.visual.block.v1` | destruction block | debris/smoke budget report, traversal aftermath frame, downgrade digest |

## Diff law
A golden diff must never compare screenshots alone.
It must include the scene-class capability and fallback packet set, hardware profile, and first divergence row.

## Blocker classes
- missing packet family;
- incompatible hardware profile;
- hidden downgrade;
- capture artifact missing or stale.

## Current posture
`document_gold / certification_corpus_closed / implementation_open`
