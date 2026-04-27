# Transport Policies Local Fields

| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| transport_policy_id | TransportPolicyId | required | stable identity of one transport policy row | unique in registry |
| framing_kind | FramingKind | required | packet framing mode | must use declared enum |
| max_payload_bytes | MaxPayloadBytes | required | upper payload bound admitted by the policy | must be finite and positive |
| retry_class | RetryClass | required | retry posture for the transport | must use declared enum and stay bounded |
| batching_window | BatchingWindow | required | maximum batching window for publication | must be explicit and finite |

## Local invariant rule
Each field above exists because `transport_policies` must publish transport policies without absorbing adjacent semantic truth.
