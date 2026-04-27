# Command Ack Progress And Terminal Result Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the singular lifecycle contract for engine responses that travel back through sdk to tooling and editor.

## Lifecycle states
| state | meaning | mandatory fields |
|---|---|---|
| `ack.accepted` | command was accepted for legal processing | `command_id`, `route_id`, `accepted_at`, `focus_target_id` |
| `ack.denied` | command was denied before execution | `command_id`, `failure_code`, `next_action_id`, `focus_target_id` |
| `progress.bound` | baseline / replay / artifact binding is complete | `command_id`, `baseline_id`, `artifact_ref` |
| `progress.running` | active route slice is executing | `command_id`, `route_state`, `percent_hint`, `trace_ref` |
| `partial_result` | a partial but operator-visible result is available | `command_id`, `partial_kind`, `artifact_ref`, `focus_target_id` |
| `terminal_success` | route completed legally | `command_id`, `success_code`, `artifact_ref`, `evidence_posture`, `focus_target_id` |
| `retryable_failure` | route failed but has a legal retry | `command_id`, `failure_code`, `retry_budget_left`, `next_action_id`, `focus_target_id` |
| `terminal_failure` | route failed and cannot retry | `command_id`, `failure_code`, `artifact_ref`, `next_action_id`, `focus_target_id` |

## Allowed emission ladders
| route outcome | allowed lifecycle |
|---|---|
| deny before execution | `ack.denied` |
| success after acceptance | `ack.accepted -> progress.bound? -> progress.running* -> partial_result* -> terminal_success` |
| retryable failure after acceptance | `ack.accepted -> progress.bound? -> progress.running* -> partial_result* -> retryable_failure` |
| terminal failure after acceptance | `ack.accepted -> progress.bound? -> progress.running* -> terminal_failure` |

## Lifecycle law
- every promoted command family must declare which lifecycle states it may emit;
- no command may emit a terminal state without either a success code or a failure code;
- retryable failures must name remaining budget and one next legal recovery action;
- terminal results must remain linkable to route state and evidence posture;
- `ack.denied` may not be followed by any progress or terminal state;
- `partial_result` may appear only when the operator receives a stable visible intermediate artifact.

## Packet-family duties
- command packets start the lifecycle and must carry a stable `command_id`;
- observation packets must preserve `command_id` and `trace_ref` when they feed diagnostics;
- compare, capture, recover, certify, freeze, and signoff packets must end in a terminal state with `artifact_ref`;
- every denial or failure packet must expose `failure_code`, `next_action_id`, and `focus_target_id`;
- every route family participating in replay or baseline recovery must expose compatibility and version fields.

## Artifact and trace requirements
- `artifact_ref` is mandatory for every terminal state participating in compare, capture, certification, freeze, or signoff;
- `trace_ref` is mandatory for every progress or failure state that feeds diagnostics review;
- `focus_target_id` and `next_action_id` are mandatory for every denial or failure state;
- `baseline_id` is mandatory for any lifecycle that claims compare, replay, or recover semantics.

## Current posture
`document_gold / doc_closed_impl_open`
