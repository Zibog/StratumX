# engine_net_transport

## Stack Position

L2.5. Network Runtime Services

## Primary Role

net transport.

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
| Connection Model | Runtime transport connection model. | `40_CONNECTION_MODEL.md` |
| Packet Lanes | Named packet-lane model. | `41_PACKET_LANES.md` |
| Security and Session | Session and handshake model. | `42_SECURITY_AND_SESSION.md` |
