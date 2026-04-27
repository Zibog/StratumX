# Execution Flow

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Write path
1. upper layer prepares a typed command packet or control
2. compatibility facts are read from immutable bridge snapshots
3. compatibility verdicts and legality gates are resolved through compiled lookup tables
4. transport policy selects admissible publication shape
5. ingress lane publishes toward engine L4 ordered sinks

## Read path
1. engine L4 publishes observations, metrics, handles, refs, or artifact refs through public batch or snapshot surfaces
2. L5 normalizes the publication without semantic widening
3. immutable batches, cursors, and snapshots are exposed upward by role and by budget class

## Bridge invariant
No write path may depend on mutable editor state inside L5.
No read path may duplicate engine truth into a hidden second mutable store.
