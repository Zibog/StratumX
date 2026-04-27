# Libraries

## Local record classes
- `PacketId` for `packet_id`
- `EngineSessionHandle` for `session_handle`
- `TransportPolicyId` for `transport_policy_id`
- `CompatVersionId` for `compat_version_id`
- `OpaquePacketPayload` for `payload_bytes`
- `IngressTick` for `received_at_tick`
- `PacketDecodeStatus` for `decode_status`

## Shared registries consumed
- `compat_versions` registry or lookup surface
- `transport_policies` registry or lookup surface
- `legality_gates` registry or lookup surface
- `engine_session_handles` registry or lookup surface

## Library law
`link_ingress_packets` may introduce only record classes that help publish or resolve link ingress packets. It may not invent a parallel semantic vocabulary for adjacent bridge classes.
