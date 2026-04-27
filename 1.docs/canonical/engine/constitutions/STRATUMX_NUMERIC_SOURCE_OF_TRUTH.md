# STRATUMX_NUMERIC_SOURCE_OF_TRUTH

## 1. Purpose

This document is the canonical numeric source of truth for shared StratumX numeric constants.
When a hard numeric ceiling, cadence, size, window, quota, or frozen grid is shared or repeated elsewhere in the package, this document wins.
Local documents may echo numeric values only when they cite this document as the authority. Local-only numeric constants may remain in their owning document when they are not shared and do not claim to override this document.

## 2. Governance Rule

- every shared cross-document numeric constant belongs here;
- local documents may reference numeric rows from this document, but may not invent conflicting values for shared constants;
- when a new shared hard number is introduced anywhere in the package, this document must be updated in the same canon revision;
- if a local document and this document disagree on a shared constant, this document is authoritative and the local document is in drift;
- local-only numeric constants are legal only when they stay in their owning document and are not reused as shared package constants.

## 3. Profile Core Constants

| Profile | Fixed cadence | Frame or tick budget | Notes |
|---|---:|---:|---|
| `interactive-60` | 60 Hz presentation | 16.67 ms/frame | one local presentation authority |
| `listen-host-60` | 60 Hz presentation | 16.67 ms/frame | local presentation plus host-side remote burden |
| `headless-20` | 20 Hz authoritative sim | 50.00 ms/tick | no presentation ownership |

## 4. Chunk and Region Constants

| Item | Canonical value |
|---|---:|
| chunk edge length | 32 m |
| vertical subdivision slab height | 16 m |
| default same-tick halo width | 1 chunk |
| maximum explicit field-solve halo width | 2 chunks |
| same-tick dirty propagation width | 1 halo |
| cross-chunk destruction spill ceiling | at most 1 halo per authoritative tick |
| activation migration ceiling | <= 4 chunk ownership moves per authority step |
| metadata migration ceiling | <= 256 KiB metadata and <= 2,048 membership remaps per activation step |

Interior or underground authored space must still project into canonical `(chunk_x, chunk_y, slab_z)` addressing.

## 5. Queue Constants

| Queue | Depth ceiling | Age or flush ceiling |
|---|---:|---:|
| apply queue | <= 16,384 staged records per segment and <= 65,536 aggregate per authoritative tick | <= 1 authoritative tick |
| transfer completion queue | <= 4,096 records per frame | <= 2 frames on presentation profiles and <= 2 ticks on headless |
| per-connection publication queue | <= 1,024 records and <= 256 KiB queued payload per peer | control publications <= 100 ms; bulk publications <= 500 ms |
| low-latency presentable-frame queue | <= 1 queued presentable frame | <= 1 frame |
| immediate IO queue | <= 256 page jobs | <= 2 frames cancel-to-visible |
| predicted IO queue | <= 512 page jobs | <= 4 frames cancel-to-visible |
| maintenance IO queue | <= 256 page jobs | <= 8 frames cancel-to-visible |

## 6. Allocator Class Ceilings

### 6.1. Per-class ceilings

| Class | `interactive-60` | `listen-host-60` | `headless-20` |
|---|---:|---:|---:|
| `frame-scratch` | 64 MiB | 64 MiB | 8 MiB |
| `tick-scratch` | 96 MiB | 128 MiB | 128 MiB |
| `cell-scratch` | 8 MiB per active chunk / 128 MiB aggregate | 8 MiB per active chunk / 160 MiB aggregate | 8 MiB per active chunk / 192 MiB aggregate |
| `session-persistent` | 384 MiB | 512 MiB | 512 MiB |
| `streaming-resident` | 1,280 MiB | 1,536 MiB | 1,152 MiB |
| `staging-backed` | 192 MiB | 256 MiB | 64 MiB |

### 6.2. Pressure-response latency

| Item | Canonical value |
|---|---:|
| pressure ladder entry after class breach | <= 2 realtime frames or <= 2 headless ticks |

## 7. Residency Windows

| Transition | `interactive-60` / `listen-host-60` | `headless-20` |
|---|---:|---:|
| `cold -> warm` | 2 consecutive locality hits or 1 startup bind | 2 consecutive locality hits or 1 startup bind |
| `warm -> hot` | 1 current-frame bind or 2 consecutive near-window hits within 8 frames | 1 current-tick bind or 2 consecutive near-window hits within 4 ticks |
| `hot -> warm` | 24 frames with no hot reference | 10 ticks with no hot reference |
| `warm -> cold` | 96 frames with no warm reference | 40 ticks with no warm reference |
| `quarantined -> legal publish` | fence/completion visible + 2 frame grace | fence/completion visible + 2 tick grace |
| promotion debt ceiling | <= 2 frames | <= 2 ticks |
| eviction batch ceiling | <= 64 MiB or <= 512 objects per frame | <= 96 MiB or <= 768 objects per tick |

## 8. Stream Activation and Transfer Constants

### 8.1. Request classes

| Class | Count ceiling | `interactive-60` | `listen-host-60` | `headless-20` |
|---|---:|---:|---:|---:|
| immediate | <= 256 in-flight requests | <= 16 MiB | <= 24 MiB | <= 24 MiB |
| predicted | <= 1,024 in-flight requests | <= 64 MiB | <= 96 MiB | <= 96 MiB |
| maintenance | <= 768 in-flight requests | <= 48 MiB | <= 64 MiB | <= 64 MiB |
| aggregate requests | <= 2,048 in-flight requests | same | same | same |

### 8.2. Upload staging ceilings

| Item | Canonical value |
|---|---|
| upload bytes per presentation frame | <= 32 MiB p95 and <= 64 MiB p99 |

### 8.3. Decode ceilings

| Profile | CPU ceiling | Compressed bytes ceiling |
|---|---:|---:|
| `interactive-60` | <= 0.50 ms p95 / <= 0.75 ms p99 per frame | <= 8 MiB per frame |
| `listen-host-60` | <= 0.75 ms p95 / <= 1.00 ms p99 per frame | <= 12 MiB per frame |
| `headless-20` | <= 1.00 ms p95 / <= 1.50 ms p99 per tick | <= 16 MiB per tick |

## 9. Network Constants

| Item | Canonical value |
|---|---:|
| per-connection ack window | <= 256 sequence ids or ranges |
| outstanding reliable control packets | <= 64 |
| outstanding reliable bulk packets | <= 128 |
| resend horizon for control lane | <= 500 ms |
| resend horizon for bulk lane | <= 1,500 ms |
| ack-state bytes per peer | <= 32 KiB |
| resend-buffer bytes per peer | <= 256 KiB |
| delivery-state bytes per peer | <= 512 KiB on `listen-host-60` and `headless-20`, <= 384 KiB on `interactive-60` |
| input-history bytes per peer | <= 64 KiB |
| aggregate host history budget | <= 1 MiB on `listen-host-60`; <= 2 MiB on `headless-20` |
| concurrent remote sessions | <= 8 on `interactive-60`, <= 16 on `listen-host-60`, <= 32 on `headless-20` |
| authenticated session-state bytes per peer | <= 128 KiB on `interactive-60`, <= 192 KiB on `listen-host-60` and `headless-20` |
| handshake timeout | 5 s |
| reconnect backoff | 1 s min, exponential, 8 s max |
| disconnect linger | <= 2 s |

### 9.1. Default quantization grids

| Quantization class | Canonical numeric grid |
|---|---|
| exact integer | exact `u32`/`i32`/`u64` as declared by field contract |
| bounded fixed-point position | signed 24-bit fixed-point at 1 cm step over local range <= +/-83,886.07 m, with default regional export clamp <= +/-4,096 m |
| bounded angular | 16-bit unsigned angle over `[0, 2π)` |
| bounded normalized scalar | 12-bit unsigned normalized scalar over `[0, 1]` |
| bounded packed bitfield | 8, 16, or 32 bits only |
| lossy approximate | explicit per-field class, minimum 10-bit mantissa equivalent or better |

## 10. Replay Constants

| Item | `interactive-60` / `listen-host-60` | `headless-20` |
|---|---:|---:|
| full checkpoint cadence | every 100 authoritative ticks | every 50 authoritative ticks |
| checkpoint compare cadence | every 20 authoritative ticks | every 10 authoritative ticks |
| checkpoint size ceiling | <= 8 MiB | <= 12 MiB |
| replay log bytes per tick ceiling | <= 64 KiB | <= 96 KiB |
| save boundary package ceiling | <= 12 MiB | <= 16 MiB |
| join handoff replay payload ceiling | <= 12 MiB | <= 16 MiB |

## 11. Imaging and Acoustics Constants

### 11.1. Imaging visibility

| Item | Canonical value |
|---|---:|
| instance-cluster size | <= 256 instances |
| cluster descriptor payload | <= 64 KiB |
| submission buckets | <= 128 buckets per frame |
| occlusion feedback lag | <= 2 frames |
| visibility-result staleness | <= 1 frame |

### 11.2. Acoustics runtime buffering

| Item | `interactive-60` / `listen-host-60` | `headless-20` |
|---|---:|---:|
| active voices | <= 96 | <= 64 |
| decode concurrency | <= 2 | <= 2 |
| critical cue buffers | <= 16 MiB aggregate | <= 16 MiB aggregate |
| ambience buffers | <= 32 MiB aggregate | <= 16 MiB aggregate |

## 12. Diagnostics Constants

| Item | Canonical value |
|---|---:|
| diagnostics CPU overhead on `interactive-60` / `listen-host-60` | <= 3% p95 and <= 5% p99 |
| diagnostics CPU overhead on `headless-20` | <= 5% p95 and <= 8% p99 |
| explicit GPU readback diagnostics window | <= 1 readback batch every 30 realtime frames; <= 8 MiB per readback batch |
| diagnostics memory shadow ceiling | <= 64 MiB per active profile |

## 13. Numeric Authority Annotation Rule

Any local document that repeats a shared hard number must include a line of the form:

`Numeric Authority: STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md §<section>/<row>`

Local-only numbers must be marked:

`Numeric Authority: local-only numeric`

The canonical validator contract is frozen by `STRATUMX_NUMERIC_VALIDATION_RULE.md`.


## 14. Validation Binding

Package-contained numeric governance for this stack marker is registered by:
- `STRATUMX_NUMERIC_VALIDATION_RULE.md`
- `implementation/STRATUMX_NUMERIC_VALIDATOR_ARTIFACT.md`
- `implementation/STRATUMX_NUMERIC_VALIDATOR_RESULT.md` run `NUMVAL-R13-01`

Shared constants repeated in local docs are package-proven only within that validator scope.
