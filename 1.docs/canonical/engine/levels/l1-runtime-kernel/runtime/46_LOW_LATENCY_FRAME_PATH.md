# Low Latency Frame Path

## Role

Runtime-owned low-latency frame path for realtime profiles.

## Path Matrix

| Stage | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| extraction handoff | bounded one-frame visibility freshness | reduce decorative extraction first | stale-frame reuse beyond ceiling |
| upload staging | bounded by transfer/upload ceilings | drop optional uploads first | upload spill into hidden queued frames |
| present queue | <= 1 queued frame and <= 1 frame queue age | degrade quality first | queue creep |
| readback/diagnostics | diagnostics law only | cancel diagnostics first | readback in critical path |
| publication windows | runtime-owned and fixed | defer noncritical publication | service-owned publish pacing |

## Rule

Low-latency legality is violated by queue creep, stale visibility, or hidden upload spill even when frame p95 still looks legal.

## Binding Table

| Binding | Canonical source |
|---|---|
| queue and queue-age ceilings | `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` |
| visibility freshness | `STRATUMX_RENDER_VISIBILITY_AND_TRANSFER_LAW.md` |
| upload staging posture | `levels/l3.1-synthesis-systems/imaging/45_UPLOAD_STAGING.md` |
| diagnostics readback | `STRATUMX_DIAGNOSTICS_CEILING_LAW.md` |

## Illegal Patterns

- queue-age compliance faked by rotating hidden buffers;
- upload debt hidden in later frames;
- low-latency path widened by diagnostics or replay work.

## Latency-Rot Failure Order

When low-latency pressure rises, optional uploads, decorative passes, and diagnostics readbacks yield before queue age, frame count, or visibility freshness are allowed to drift.

## Local Operating Law

The low-latency frame path is a hard boundary surface.
Every consumer on that path must be able to fail closed without creating hidden queue or stale-data debt.
