# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| world_state | Authoritative root state | Required | Canonical root holding authoritative world truth | engine_world |
| snapshot | Immutable world snapshot | Required | Versioned, ordered, profile-visible | engine_world |
| read_model | Versioned read view | Required | Aligned to execution windows and deterministic access | engine_world |
| apply_segment | Apply segment | Required | Region-scoped, island-scoped, family-tagged | engine_world |
| segment_count | Segment count per tick | Required | <= 256 segments per authoritative tick | engine_world |
| family_fanout | Family fan-out per segment | Required | <= 8 families per segment | engine_world |
| publish_passes | Publication order passes | Required | <= 2 ordered passes across all segments | engine_world |

## Invariant Rules

- Snapshots are immutable read products for runtime, critical families, sync, and synthesis
- Snapshots serve deterministic replay and divergence checkpoints
- Snapshots remain versioned, ordered, and profile-visible
- Apply is segmented: region-scoped, island-scoped, family-tagged, and publication-safe
- Whole-world monolithic integration is illegal when legal segmented apply exists
- Recursive segment spawning inside apply is illegal
- Publication order depending on hash-map iteration or worker race is illegal
- Micro-segmentation used to hide monolithic apply cost is illegal
- Apply boundary is the single authoritative boundary where staged changes become world truth
