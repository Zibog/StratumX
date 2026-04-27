# L5 Synchronization Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

`L6` synchronizes with `L5` through typed bridge surfaces only:
- handles
- refs
- artifact refs
- immutable bridge snapshots
- observation batches
- metric batches
- compatibility facts
- compatibility verdict tables
- legality outputs
- ingress command publication
- ingress binding and execution controls
- bridge epoch / invalidation markers

`L6A`, `L7`, and `L7A` consume `L5` only indirectly through `L6` unless an explicit upper-stack bridge surface says otherwise.
`L6` must not reshape `L5` facts into hidden mutable mirrors. It must normalize them into canonical planes, batches, and refs.
