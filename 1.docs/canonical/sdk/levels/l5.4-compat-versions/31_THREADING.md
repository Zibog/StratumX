# Threading

## Concurrency posture for compat versions
- publication and mutation of compat_version_id remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as compat_version_id, wire_version, schema_generation without creating a second writer
- no background task may rewrite already-published compat versions rows

## Forbidden concurrency patterns
- hidden mutable mirrors of compat versions
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
