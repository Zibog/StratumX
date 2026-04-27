# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| mutation_buffer | Batched mutation carrier | Required | Must support merge/coalesce rules | engine_storage_mutation |
| idempotence_class | Idempotence classification | Required | Must be explicit for batch ordering | engine_storage_mutation |
| batch_order | Ordering constraint | Required | Must scale by batches not micro-writes | engine_storage_mutation |
| tombstone_window | Compaction window | Optional | Must support tombstone/compaction | engine_storage_mutation |
| deferred_write_queue | Write deferral queue | Required | Keeps direct world mutation out of compute | engine_storage_mutation |
| change_set | Grouped changes | Required | Structural and data changes for controlled integration | engine_storage_mutation |
| apply_payload | Ordered batch | Required | Family-tagged, region-tagged for segmented apply | engine_storage_mutation |
| family_tag | Family classification | Required | For segmented apply routing | engine_storage_mutation |
| region_tag | Region classification | Required | For regional/island-scoped integration | engine_storage_mutation |

## Invariant Rules

- Mutation buffers must support merge/coalesce rules
- Apply cost must scale by batches instead of random micro-writes
- Direct world mutation during compute phases is forbidden
- Apply payloads must not require whole-world monolithic integration when regional or island-scoped integration is legal
- Batch ordering must respect idempotence classes
