# Trace, Reason, and Diagnostics Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Expose engine publications, route diagnostics, reason chains, blocker traces, and lifecycle state to the operator without stealing runtime truth.

## Promoted command inventory
| class | button id | tooling route | sdk family | engine truth owner | primary outcome |
|---|---|---|---|---|---|
| inspect | `btn.trace.reason.inspect` | `route.trace.reason.inspect.v1` | `packet.trace.reason.inspect.v1` | engine `76` + engine `97` | reason-chain drilldown |
| inspect | `btn.trace.failure.overlay` | `route.trace.failure.overlay.v1` | `packet.trace.failure.overlay.v1` | engine `76` + engine `97` | failure overlay |
| capture | `btn.trace.capture.bundle` | `route.trace.capture.bundle.v1` | `packet.trace.capture.bundle.v1` | engine `76` + engine `97` | retained trace bundle |
| follow | `btn.trace.follow.next_action` | `route.trace.follow.next_action.v1` | `packet.trace.follow.next_action.v1` | engine `76` + engine `97` | next-action focus handoff |

## Mandatory runtime surfaces
- **Command Runtime Monitor** — active `command_id`, owner lab, route id, sdk family, engine owner, current lifecycle state;
- **Route/Packet Inspector** — normalized route payload, packet payload, compatibility version, failure/success code;
- **Lifecycle Timeline** — emitted sequence from `ack.accepted` or `ack.denied` through terminal state;
- **Focus/Recovery Viewer** — current focus target, next legal action, retry budget, recover anchor;
- **Artifact/Evidence Link Panel** — retained artifact refs, trace refs, baseline ids, compare digests, evidence bundle links.

## Overlay families
- `overlay.trace.owner`
- `overlay.trace.failure_family`
- `overlay.trace.route_state`
- `overlay.trace.lifecycle_state`
- `overlay.trace.next_action`

## Inspector fields
- `command_id`
- `trace_id`
- `route_id`
- `packet_family_id`
- `owner_package`
- `failure_code`
- `success_code`
- `next_action_id`
- `focus_target_id`
- `artifact_ref`
- `baseline_id`
- `route_state`
- `retry_budget_left`

## Compare and capture law
This lab may use only the compare/capture modes declared in root `75`, root `76`, and sdk `77`.
Any capture or compare action must lower through tooling and return a retained artifact or a denial code.

## Focus rules
- success -> `103` or `105` depending on artifact posture;
- retryable failure -> owning production lab with one explicit next legal action;
- terminal failure -> `109` only if freeze review is already active and blocker trace is attached;
- no trace-only success may bypass compare/capture when the owning pack is freeze-relevant.

## Evidence duties
If this lab participates in certification, it must point to:
- one retained artifact;
- one terminal result code;
- one next legal action;
- one focus target;
- one baseline id when compare or recover semantics are in play.

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
