# Runtime Operator Diagnostics Publication Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file freezes the engine-side truth for runtime diagnostics publication toward operator surfaces.
Upper layers may observe and route this truth.
Upper layers may not redefine publication phases, event ids, or failure semantics owned here.

## Truth objects
| Truth object | Purpose |
|---|---|
| `operator_diagnostics_stream` | authoritative event stream for runtime operator diagnostics |
| `runtime_health_snapshot` | exact health snapshot for one scope/tier/time slice |
| `operator_blocker_state` | current blocker set with severity and recovery posture |
| `diagnostics_publication_cursor` | retained publication continuity state |
| `diagnostics_artifact_contract` | engine-owned artifact obligation state |

## Runtime phases
1. collect domain-local diagnostics carriers
2. classify severity and failure family
3. resolve publication scope and tier
4. bind focus target and recovery action
5. publish observation packet
6. retain or prune publication cursor

## Publication events
- `event.runtime.diagnostics.published`
- `event.runtime.diagnostics.blocker_opened`
- `event.runtime.diagnostics.blocker_cleared`
- `event.runtime.diagnostics.capture_offered`
- `event.runtime.diagnostics.publication_pruned`

## Carriers
- `carrier.runtime_health`
- `carrier.operator_blocker`
- `carrier.capture_offer`
- `carrier.old_floor_pressure`

## Failure classes
- `diag.owner_missing`
- `diag.scope_unresolved`
- `diag.focus_unresolved`
- `diag.publication_cursor_gap`
- `diag.capture_offer_unbound`

## Degrade ladder
| Pressure axis | First degrade | Second degrade | Forbidden hidden degrade |
|---|---|---|---|
| CPU | lower publication frequency for info severity | prune non-blocking info history | blocker severity downgrade |
| GPU | collapse expensive preview attachments | disable non-essential thumbnail generation | removing blocker overlays silently |
| RAM | shorten retained info history | compact non-baseline bundles | pruning failed-run blocker bundle |
| disk/io | delay non-essential copies | compress retained info bundles | dropping artifact ref from terminal result |

## Evidence obligations
Every blocker-bearing or capture-bearing publication must preserve:
- `publication_event_id`
- `truth_owner_id`
- `focus_target_id`
- `artifact_ref` when a retained artifact exists
- `first_failure_code` or blocker-cleared success code

## Coupling boundaries
- publishes into `sdk/71`, `sdk/73`, and `sdk/76`;
- may read threshold law from root resource spine;
- may not consume editor focus policy directly;
- may not invent tooling route ids.

## Recovery actions
- `action.diag.republish_after_owner_sync`
- `action.diag.rebind_focus_target`
- `action.diag.reopen_capture_offer`
- `action.diag.restore_failed_bundle`
