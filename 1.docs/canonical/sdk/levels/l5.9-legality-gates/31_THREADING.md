# Threading

## Concurrency posture for legality gates
- publication and mutation of legality_gate_id remain ordered according to the semantic rules of this level
- readers may fan out immutable records such as legality_gate_id, gate_name, applies_to_control_kinds without creating a second writer
- no background task may rewrite already-published legality gates rows

## Forbidden concurrency patterns
- hidden mutable mirrors of legality gates
- cross-session or cross-scope reordering that breaks declared cursors
- unsignaled cancellation or supersede of published rows
