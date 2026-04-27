# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| realtime_profile | Profile configuration | Required | Interactive-60 or listen-host-60 envelope | engine_runtime_realtime |
| presentation_cadence | Frame pacing | Required | Fixed 60 Hz, 16.67 ms hard budget | engine_runtime_realtime |
| frame_budget | Time budget | Required | 16.67 ms hard budget, no hidden wider entitlement | engine_runtime_realtime |
| presentable_queue | Frame queue | Required | <= 1 queued frame | engine_runtime_realtime |
| visibility_freshness | Visibility window | Required | <= 1 stale frame | engine_runtime_realtime |
| presentable_frame | Frame output | Required | Runtime-owned publication | engine_runtime_realtime |
| audio_submission | Audio output | Required | Runtime-owned publication window | engine_runtime_realtime |
| remote_publication | Network output | Required | Runtime-owned publication window | engine_runtime_realtime |
| replay_marker | Replay output | Optional | Bounded windows only | engine_runtime_realtime |
| diagnostics_readback | Diagnostics output | Optional | Diagnostics law only, not in critical path | engine_runtime_realtime |
| role_binding | Role classification | Required | Interactive-60 or listen-host-60 | engine_runtime_realtime |
| degradation_order | Degradation policy | Required | Runtime-owned degradation only | engine_runtime_realtime |

## Invariant Rules

- Presentation cadence is fixed 60 Hz (not variable)
- Frame budget is 16.67 ms hard budget
- Presentable queue <= 1 queued frame (no multi-frame latency reservoir)
- Visibility freshness <= 1 stale frame (no long-lived stale visibility reuse)
- Must pass mixed-pressure-realtime-a proof
- Frame ownership is runtime-owned only (no service-owned present loop)
- Frame cadence is runtime-owned and single-path
- Low-value or degradable work yields before low-latency path legality is crossed
- Presentation policy may reduce decorative quality but may not borrow world, network, replay, or residency reserve
- Realtime profile must select either interactive-60 or listen-host-60 (may not merge envelopes)
- Role widening after bootstrap is illegal
- Role binding is startup-owned and seals queue, visibility, network, replay, and simulation-tier entitlements before launch
- listen-host-60 may add remote burden only inside host-owned reserve
- Realtime outputs remain bounded publications (no queue debt, authority ownership shift, or multi-frame shadow buffers)
