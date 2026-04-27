# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| phase_state | Phase classification | Required | Ingress, read, compute, resource, authority-sync, stage, apply, egress, diagnostics | engine_runtime |
| execution_window | Scheduled window | Required | Runtime-owned scheduling authority | engine_runtime |
| scheduler | Global scheduler | Required | Worker activation, phase barriers, queue arbitration | engine_runtime |
| apply_queue | Apply staging queue | Required | <= 16,384 records per segment, <= 65,536 aggregate, <= 1 tick age | engine_runtime |
| apply_segment | Apply segment | Required | Bounded by segment count and fan-out | engine_runtime |
| publication_order | Publication ordering | Required | Deterministic inside runtime phase law | engine_runtime |
| transfer_completion_queue | Transfer queue | Required | <= 4,096 records, <= 2 frames or ticks age | engine_runtime |
| connection_publication_queue | Per-connection queue | Required | <= 1,024 records, <= 256 KiB per peer, control <= 100ms, bulk <= 500ms | engine_runtime |
| presentable_frame_queue | Low-latency frame queue | Required | <= 1 frame depth, <= 1 frame age | engine_runtime |
| frame_pacing | Pacing decision | Required | Runtime-owned pacing authority | engine_runtime |
| visibility_freshness | Visibility window | Required | Bounded one-frame freshness | engine_runtime |
| upload_staging | Upload staging | Required | Bounded by transfer/upload ceilings | engine_runtime |

## Invariant Rules

- Global scheduling ownership belongs to engine_runtime only
- No other crate may create competing global scheduler
- Queue ceilings obey numeric-source depth and age ceilings
- Work stealing legal only inside runtime-owned scheduler law
- Broad locks across compute phases are illegal
- Apply ownership belongs to runtime/world only
- Direct mutation outside apply is illegal
- Hidden family-local queues outliving runtime visibility are illegal
- Low-latency legality violated by queue creep, stale visibility, or hidden upload spill
- Low-latency frame path is hard boundary surface
- Every consumer on low-latency path must fail closed without creating hidden queue or stale-data debt
- Publication windows are runtime-owned and fixed
