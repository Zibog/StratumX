# Frame Resource Policy

## Role

Local operating policy for frame-scoped imaging resources.

## Policy Matrix

| Resource concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| transient render resources | bounded by frame-resource ceilings | drop optional pass first | hidden persistent transient set |
| submission buckets | bounded by numeric source | collapse optional buckets first | bucket explosion |
| failure posture | fail closed on required resource breach | degrade optional path first | silent widening |
| illegal paths | render path may not borrow replay/network/world reserve | refuse path | cross-domain reserve theft |

## Binding Table

| Binding | Canonical source |
|---|---|
| transient resource ceilings | `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` |
| frame-path legality | `levels/l1-runtime-kernel/runtime/46_LOW_LATENCY_FRAME_PATH.md` |
| diagnostics posture | `STRATUMX_DIAGNOSTICS_CEILING_LAW.md` |

## Per-Path Failure Posture

| Resource path | Preserve first | Degrade first |
|---|---|---|
| required transient resources | legal frame completion | optional passes |
| submission buckets | bounded submission order | decorative bucket granularity |
| diagnostics overlays | frame legality | diagnostics first |

## Illegal Path Matrix

| Illegal path | Why illegal |
|---|---|
| persistent transient set | converts frame resources into session resources |
| bucket explosion | creates hidden submit-side queue debt |
| borrowing replay or network reserve | cross-domain reserve theft |

## Local Operating Law

Frame resources are frame-scoped by law.
No local policy may normalize persistent growth or convert transient debt into tolerated background state.
