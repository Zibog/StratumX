# Batch And Cursor Publication Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Canonical rule
Observations and metrics are published as immutable batches, not as unbounded mutable object streams.
Consumption occurs through bounded monotonic cursors.

## Batch law
- each batch has one stable batch id
- each batch is immutable after publication
- fanout readers may share the same batch without hidden mutation
- batch-local anchors may point to refs, metrics, or artifacts without becoming ownership transfer

## Cursor law
- each reader cursor is monotonic within one bridge epoch
- cursor rewind is illegal unless an explicit replay class allows it
- cursor invalidation must be signaled by bridge epoch change
- cursor lag must remain observable under pressure

## Forbidden
- mutable shared event queues as the canonical read surface
- read-side semantic widening during fanout
- unbounded cursor rewind without declared replay law
