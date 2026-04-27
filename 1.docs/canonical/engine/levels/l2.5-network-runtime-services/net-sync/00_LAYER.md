# engine_net_sync

## Stack Position

L2.5. Network Runtime Services

## Primary Role

net sync.

## Canonical Scope

Runtime network-service ownership local to this crate.

## Boundary Rationale

This crate exists so transport, sync, and latency-control ownership stay explicit and do not collapse into runtime, gameplay, or SDK layers.

## Canonical Consumers

- higher service layers and startup where justified by local contracts.

## Downward Dependencies

See `20_DEPENDENCIES.md`.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Interest Model | Connection-scoped relevance model. | `40_INTEREST_MODEL.md` |
| Snapshot and Delta | Authoritative sync product model. | `41_SNAPSHOT_AND_DELTA.md` |
| Ack and Delivery | Ack and ordered-delivery visibility model. | `42_ACK_AND_DELIVERY.md` |
