# Fields

## Canonical field inventory
| Field | Type | Required | Role | Invariants |
|---|---|---|---|---|
| transport_policy_id | TransportPolicyId | required | stable identity of one transport policy row | unique in registry |
| framing_kind | FramingKind | required | packet framing mode | must use declared enum |
| max_payload_bytes | MaxPayloadBytes | required | upper payload bound admitted by the policy | must be finite and positive |
| retry_class | RetryClass | required | retry posture for the transport | must use declared enum and stay bounded |
| batching_window | BatchingWindow | required | maximum batching window for publication | must be explicit and finite |

## No hidden store law
All semantic truth in `transport_policies` must be visible as one of the declared fields above or in a declared lower-layer dependency. Hidden caches, shadow graphs, or side mirrors are illegal.
