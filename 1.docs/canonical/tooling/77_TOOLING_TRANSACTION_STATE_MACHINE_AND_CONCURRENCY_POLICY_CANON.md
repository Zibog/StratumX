# Tooling Transaction State Machine and Concurrency Policy Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file is the authoritative execution-grade law for tooling transaction lifecycle and concurrency.
It is the only active truth for transaction/concurrency policy.

## Required intent fields
- `request_id`
- `route_id`
- `concurrency_scope`
- `retry_family`
- `rollback_anchor?`
- `expected_result_kind`
- `focus_target_id`

## State machine
| state_id | Meaning | Concurrency consequence |
|---|---|---|
| `state.received` | intent accepted into tooling | slot not reserved yet |
| `state.validated` | guards and disabled reasons checked | conflicting scopes still reject |
| `state.normalized` | schema and scope normalized | normalized payload locked |
| `state.bound` | route, sdk family, and owner resolved | scope reservation begins |
| `state.executing` | active work | exclusive/shared/farm policy enforced |
| `state.partial_result` | partial publication allowed | scope remains reserved |
| `state.retryable_failure` | bounded retry legal | reservation may remain or be softened |
| `state.rolled_back` | rollback finished | prior reservation released |
| `state.recovered` | recovery rerun completed | new reservation lineage preserved |
| `state.completed` | terminal success | scope released |
| `state.terminal_failure` | terminal failure | scope released, blockers retained |

## Concurrency scopes
| scope | Meaning | Illegal with |
|---|---|---|
| `scope.single_route` | one route instance only | another same-route execution |
| `scope.shared_read` | multiple read-only compares/captures | any mutation class |
| `scope.domain_farm` | bounded farm for certify/preview | exclusive author/bind/recover |
| `scope.freeze_serial` | freeze/signoff path only | any other freeze/signoff |

## Retry and rollback law
- retries are bounded by family from root `57`;
- rollback anchor is mandatory for recover and rollback-capable author routes;
- terminal failure must preserve failure code, focus, next action, and artifact lineage;
- concurrency may not hide a conflict behind silent queueing when the user needs a blocker code.

## Law
`tooling/80` may only redirect here and may not compete with this file.
