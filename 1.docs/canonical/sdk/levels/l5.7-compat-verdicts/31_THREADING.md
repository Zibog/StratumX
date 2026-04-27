# Threading

## Concurrency posture for compat verdicts
- publication and mutation of compat_verdict_id remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as compat_verdict_id, evaluated_profile_id, evaluated_version_id without creating a second writer
- no background task may rewrite already-published compat verdicts rows

## Forbidden concurrency patterns
- hidden mutable mirrors of compat verdicts
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
