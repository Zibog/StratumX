# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| headless_profile | Profile configuration | Required | Simulation-first execution profile, headless-20 envelope | engine_runtime_headless |
| authoritative_cadence | Tick pacing | Required | Fixed 20 Hz, 50.00 ms hard budget | engine_runtime_headless |
| tick_budget | Time budget | Required | 50.00 ms hard budget, no hidden wider entitlement | engine_runtime_headless |
| world_delta | Authoritative output | Required | Authoritative world deltas | engine_runtime_headless |
| network_publication | Network output | Required | Bounded by network ceilings | engine_runtime_headless |
| replay_artifact | Replay output | Optional | Bounded by replay ceilings | engine_runtime_headless |
| verification_artifact | Verification output | Optional | Bounded by storage ceilings | engine_runtime_headless |
| diagnostics_output | Diagnostics output | Optional | Bounded by diagnostics ceilings | engine_runtime_headless |
| role_binding | Role classification | Required | Authoritative host, dedicated server, or offline validation | engine_runtime_headless |

## Invariant Rules

- Headless profile owns no presentation queue
- Uses headless-20 memory and residency envelope
- Authoritative cadence is fixed 20 Hz (not variable)
- Tick budget is 50.00 ms hard budget
- Must pass mixed-pressure-headless-a proof
- Replay may consume only frozen reserve
- Storage and verification may not steal authoritative cadence
- Remote burden stays inside headless profile ceilings
- Headless roles may not inherit local-presentation reserve
- Host burden may not exceed mixed-pressure headless proof
- Verification and diagnostics yield before authoritative progression
- No presentable frame path
- Headless cadence is hard operating law, not tuning hint
- All outputs remain subordinate to tick legality
- Startup role bind must be explicit and frozen before launch
