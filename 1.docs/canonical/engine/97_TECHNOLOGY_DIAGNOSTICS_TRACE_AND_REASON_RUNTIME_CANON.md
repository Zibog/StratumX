# Technology Diagnostics Trace And Reason Runtime Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact engine truth for technology trace construction, reason-chain publication, diagnostics binding, and retained trace evidence.

## Truth objects
| Truth object | Purpose |
|---|---|
| `trace_graph_state` | authoritative graph of causal trace nodes |
| `reason_chain_state` | operator-facing terminal cause chain |
| `trace_event_ledger` | retained source event ledger for replay/compare windows |
| `diagnostic_envelope_state` | bound severity/failure/focus result for one trace publication |
| `trace_retention_state` | retention and compaction state for trace evidence |

## Runtime phases
1. receive source events from domain publishers;
2. merge source events into trace graph;
3. resolve terminal cause and reason chain;
4. bind diagnostics envelope;
5. publish observation and capture eligibility;
6. retain or compact according to retention class.

## Publication events
- `event.trace.graph.published`
- `event.trace.reason_chain.published`
- `event.trace.capture_eligibility.opened`
- `event.trace.compaction.performed`
- `event.trace.publication.failed`

## Failure classes
- `trace.source_event_gap`
- `trace.merge_conflict`
- `trace.reason_chain_unresolved`
- `trace.capture_contract_missing`
- `trace.compaction_forbidden`

## Degrade ladder
- first degrade: compact non-baseline trace branches outside active compare window;
- second degrade: drop non-terminal informational nodes while preserving terminal cause and first failure;
- forbidden: hiding a surviving terminal cause, dropping baseline-linked nodes, or clearing trace refs from terminal packets.

## Evidence obligations
A terminal trace publication must preserve:
- one stable `trace_ref`;
- one exact `publication_event_id`;
- one `first_failure_code` or terminal success code;
- one legal `focus_target_id`;
- artifact eligibility when capture or compare is possible.

## Coupling boundaries
- receives source events from heavy and render/audio domains;
- publishes upward only through sdk trace/capture surfaces;
- does not own editor overlays or tooling route selection;
- does not own certification pack semantics, only trace truth and evidence eligibility.

## Phase-6 obligations
- regression review must be able to resolve every blocker back to one retained `trace_ref`;
- freeze review must never proceed when the blocker trace is absent;
- region-scale validation may summarize trace density, but may not remove terminal cause lineage.

## Recovery actions
- `action.trace.rebuild_graph_from_ledger`
- `action.trace.restore_retained_branch`
- `action.trace.rebind_capture_contract`
- `action.trace.focus_terminal_cause`

## Negative law
Trace publication is invalid when:
- terminal result lacks a `trace_ref`;
- diagnostics envelope lacks a `focus_target_id`;
- capture eligibility is published without an artifact contract;
- compare-bearing publication omits replay window bounds.
