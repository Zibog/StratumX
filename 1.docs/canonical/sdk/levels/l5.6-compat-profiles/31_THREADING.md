# Threading

## Concurrency posture for compat profiles
- publication and mutation of profile_id remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as profile_id, profile_name, allowed_version_set without creating a second writer
- no background task may rewrite already-published compat profiles rows

## Forbidden concurrency patterns
- hidden mutable mirrors of compat profiles
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
