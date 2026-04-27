# Threading

## Ownership

No global threading ownership.

## Posture

Parallel reads and controlled staged writes under runtime governance.

## Parallel Units

Snapshot reads, region-aligned reads, controlled apply windows.

## Write Model

Final world mutation happens through controlled apply only.
