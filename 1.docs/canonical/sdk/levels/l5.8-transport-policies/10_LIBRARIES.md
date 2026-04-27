# Libraries

## Local record classes
- `TransportPolicyId` for `transport_policy_id`
- `FramingKind` for `framing_kind`
- `MaxPayloadBytes` for `max_payload_bytes`
- `RetryClass` for `retry_class`
- `BatchingWindow` for `batching_window`

## Shared registries consumed
- `compat_versions` registry or lookup surface
- `compat_profiles` registry or lookup surface

## Library law
`transport_policies` may introduce only record classes that help publish or resolve transport policies. It may not invent a parallel semantic vocabulary for adjacent bridge classes.
