# Connection Model

## Role

Transport/session connection ownership surface.

## Connection Matrix

| Concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| concurrent sessions | bounded by profile/network ceilings | refuse new peers before breach | hidden peer expansion |
| handshake timeout | fixed bounded window | fail closed | unbounded handshake linger |
| session-state bytes | bounded per peer and aggregate | shed oldest failed negotiation first | unlimited session shadow-state |
| reconnect/backoff | fixed canonical backoff classes | back off before retry storm | busy-loop reconnect |
| transport ownership | `engine_net_transport` only | refuse hidden transport owner | runtime or startup owning session truth |

## Security Posture

Connection state may not widen reliability, encryption, or identity posture beyond the canonical session classes defined by the transport and security docs.

## Binding Table

| Binding | Canonical source |
|---|---|
| peer/session ceilings | `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` |
| profile ownership | `levels/l4-startup/startup/44_NETWORK_ROLE_SELECTION.md` |
| security/session posture | `41_PACKET_LANES.md`, `42_SECURITY_AND_SESSION.md` |

## Illegal Patterns

- handshake states that outlive timeout law;
- reconnect storms that bypass backoff classes;
- hidden per-peer memory growth outside transport ceilings.

## Connection State Matrix

| State class | Canonical bound | Illegal widening |
|---|---|---|
| handshake state | timeout-bound | indefinite negotiation |
| authenticated session state | per-peer bounded | shadow identity cache |
| reconnect state | explicit backoff classes | busy-loop retry |

## Operational Contract

Transport owns connection truth.
Runtime and startup may bind roles, but may not own handshake state, ack windows, or session-memory growth after bootstrap.
