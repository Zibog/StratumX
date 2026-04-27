# Reflection And Occlusion

## Role

Bounded reflection/occlusion contract for acoustics.

## Matrix

| Concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| reflection classes | bounded authored classes only | collapse low-value reflections first | dynamic unbounded class growth |
| occlusion evaluation | bounded query count and cadence | approximate before breach | per-voice exhaustive ray path |
| material coupling | read material lookup outputs only | fallback deterministically | acoustics rewrites material law |
| diagnostics | diagnostics-law bounded only | drop diagnostics first | stealth acoustic probe ledger |

## Illegal Patterns

- reflection class growth outside authored classes;
- per-voice exhaustive occlusion when bounded approximation is required;
- acoustics-side ownership of material or world-query truth.

## Failure Priority Matrix

| Pressure source | Preserve first | Degrade first |
|---|---|---|
| query pressure | gameplay-critical cues | low-value reflections |
| cache pressure | bounded authored classes | optional occlusion detail |
| diagnostics pressure | runtime legality | acoustic diagnostics first |

## Local Operating Law

Reflection and occlusion remain approximation-first surfaces.
They may not silently widen into exhaustive ray paths or rewrite material lookup ownership.
