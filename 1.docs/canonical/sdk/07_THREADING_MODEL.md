# Threading Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Single-writer zones
- per-session ingress publication ordering
- per-lane legality decision publication
- per-lane policy publication
- low-churn snapshot replacement per bridge epoch

## Parallel-friendly zones
- fact reads over immutable snapshots
- verdict lookup over compiled immutable tables
- metric fanout over immutable batches
- observation fanout over immutable batches
- ref materialization over immutable projection snapshots

## Forbidden
- multi-writer mutation around one ingress lane
- hidden cross-thread ref mutation
- blocking editor-shaped work inside bridge publication
- mutable global mega-store spanning packets, controls, observations, and refs

Threading goal: keep L5 mostly wait-free for read fanout, snapshot reads, and cursor advancement while keeping write ingress narrowly ordered.
