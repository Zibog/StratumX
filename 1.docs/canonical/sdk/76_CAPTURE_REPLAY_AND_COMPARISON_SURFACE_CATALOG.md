# Capture Replay And Comparison Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the exact bridge for capture request publication, replay window publication, and compare result publication across all operator-grade domains.

## Authoritative packet families
| Packet family | Purpose |
|---|---|
| `packet.capture.request.v1` | request or describe one legal capture action |
| `packet.capture.result.v1` | terminal result of a capture route |
| `packet.replay.window.v1` | declare replay span, anchor, and restore eligibility |
| `packet.compare.request.v1` | declare a compare attempt and exact compare mode |
| `packet.compare.result.v1` | publish compare outcome, digest, and blocker data |

## Field-level schema table
| Field | Required |
|---|---|
| `capture_mode_id` | conditional |
| `compare_mode_id` | conditional |
| `baseline_ref` | conditional |
| `failed_run_ref` | conditional |
| `recovery_run_ref` | conditional |
| `window_start_tick` | conditional |
| `window_end_tick` | conditional |
| `anchor_ref` | conditional |
| `tolerance_class` | conditional |
| `digest_hash` | conditional |
| `trace_ref` | conditional |
| `artifact_ref` | conditional |
| `focus_target_id` | yes |
| `next_action_id` | yes |
| `terminal_code` | conditional |
| `first_failure_code` | conditional |

## Capture mode registry
- `capture.snapshot.state`
- `capture.snapshot.overlay`
- `capture.timeline.triplet`
- `capture.trace.bundle`
- `capture.cert.bundle`

## Compare mode registry
- `compare.hash.strict`
- `compare.tolerance.scalar`
- `compare.graph.topology`
- `compare.timeline.triplet`
- `compare.freeze.review`

## Replay contract
Replay-bearing packets must identify:
- `anchor_ref`
- `window_start_tick`
- `window_end_tick`
- `truth_owner_id`
- `scope_tag`
- `tier_tag`
- `preserved_first_failure_code` when replay is opened from failure context

## Failure families
- `compare.baseline_missing`
- `compare.digest_incomplete`
- `capture.mode_forbidden`
- `replay.window_invalid`
- `replay.anchor_unresolved`

## Compatibility and normalization law
- capture and compare mode ids are canonical registry ids and may not be aliased by editor labels;
- a packet may normalize time units or scalar units, but must preserve explicit unit metadata;
- silent promotion from single-run capture to triplet capture is forbidden.

## Evidence duties
A terminal compare or terminal capture result must make evidence append possible by carrying:
- `artifact_ref`
- `trace_ref` when diagnostics context exists
- `focus_target_id`
- `next_action_id`
- `terminal_code` or `first_failure_code`

## Failure and focus preservation law
No compare or capture packet is valid when:
- `focus_target_id` is absent;
- `next_action_id` is absent on terminal states;
- `baseline_ref` is absent for compare modes that require a baseline;
- `failed_run_ref` is absent when the route claims recovery comparison.
