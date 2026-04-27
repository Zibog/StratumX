# Fields

## Canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| packet_id | PacketId | required | stable identity for one ingress packet | unique within one bridge session and never reused |
| session_handle | EngineSessionHandle | required | bridge session that received the packet | must resolve through `engine_session_handles` |
| transport_policy_id | TransportPolicyId | required | transport legality profile applied before decode | must resolve through `transport_policies` |
| compat_version_id | CompatVersionId | required | declared bridge protocol version | must resolve through `compat_versions` |
| payload_bytes | OpaquePacketPayload | required | opaque L4 packet body before semantic split | may not be mutated after publication |
| received_at_tick | IngressTick | required | ordered receive clock for replay and dedupe | monotonic per session |
| decode_status | PacketDecodeStatus | required | packet parse state before semantic routing | must remain in declared enum only |

## No hidden store law
All semantic truth in `link_ingress_packets` must be visible as one of the declared fields above or in a declared lower-layer dependency. Hidden caches, shadow graphs, or side mirrors are illegal.
