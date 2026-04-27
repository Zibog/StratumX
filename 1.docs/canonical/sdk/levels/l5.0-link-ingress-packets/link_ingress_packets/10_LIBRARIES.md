# Link Ingress Packets Local Libraries

## Local vocabulary
- `PacketId` for `packet_id`
- `EngineSessionHandle` for `session_handle`
- `TransportPolicyId` for `transport_policy_id`
- `CompatVersionId` for `compat_version_id`
- `OpaquePacketPayload` for `payload_bytes`
- `IngressTick` for `received_at_tick`
- `PacketDecodeStatus` for `decode_status`

## Audit rule
Every local type above must exist to support this semantic class, not a neighboring one.
