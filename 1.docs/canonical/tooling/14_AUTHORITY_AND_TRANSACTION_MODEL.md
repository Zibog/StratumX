# Authority And Transaction Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

`L6 authority_core` is tiny, explicit, single-writer at the commit point, and never bulk-heavy.
Every editor mutation must enter through `command_envelopes` and commit through `transaction_ledger`.
Undo, redo, assistant apply, assistant revert, prefab apply/revert, unpack, variant creation, layer assignment, and batch-authoring operations remain inside transaction law only.

## Authority core minimum responsibilities
- stable authoring ids and parent/child edges;
- component attachment and component-presence truth;
- prefab instance links, nested links, and local override declarations;
- layer/data-layer memberships and streaming-policy assignments;
- package/workspace-scoped authoring ownership refs;
- mutation epochs and version counters.

## Command envelope minimum fields
- `command_id`
- `command_class`
- `target_scope`
- `target_ref_set`
- `authority_touch_class`
- `previewability_class`
- `approval_class`
- `budget_class`
- `origin_class`
- `undo_class`

## Transaction ledger minimum outputs
- `transaction_id`
- `admission_verdict`
- `commit_order`
- `rollback_binding`
- `snapshot_invalidation_set`
- `index_invalidation_set`
- `derived_invalidation_set`
- `artifact_invalidation_set`
- `stream_publication_set`
- `evidence_record`

## Law
Bulk domain data must live as immutable snapshots, rebuildable indices, deterministic artifacts, or bounded streams, not inside `authority_core`.
The ledger is where legality is proven, not hidden in panel code, plugin code, or assistant code.
