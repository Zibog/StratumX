# First Playable Slice And Operator Progress Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the public packets that report progress, blockers, readiness, and retained results for the narrow proof region.

## Required families
| Family | Required fields | Primary consumers |
|---|---|---|
| `packet.first_playable_progress.v1` | `lane_step_id`, `status`, `focused_surface_id`, `blocking_family`, `artifact_ref` | editor `123`, tooling `87`, `92` |
| `packet.first_playable_readiness.v1` | `world_open`, `terrain_visible`, `sky_bound`, `proof_pocket_ready`, `capture_ready`, `first_blocker_family` | editor `123`, `124`, tooling `87–88` |
| `packet.release_seal_review.v1` | `index_sync_status`, `ledger_sync_status`, `coverage_sync_status`, `open_truth_count`, `seal_verdict`, `artifact_ref` | editor `124`, tooling `88` |

## Surface law
Progress may be partial.
Readiness may be blocked.
Neither surface may silently collapse into a generic green status.
