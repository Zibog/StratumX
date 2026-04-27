# STRATUMX_SDK_L5_CONCURRENCY_CONSTITUTION

## Scope
This constitution constrains concurrency behavior for bridge publications.

## Binding laws
- bridge snapshots are immutable once published;
- ordered lanes, batches, epochs, and cursors define visibility rather than hidden locks;
- concurrency semantics must remain externally explainable through published cursors/epochs, not implicit races.

## Audit checks
- no `L5` local doc relies on hidden scheduling state;
- ordering/freshness is expressed through typed epochs or cursors;
- readers can consume immutable views without reverse-engineering writer timing.
