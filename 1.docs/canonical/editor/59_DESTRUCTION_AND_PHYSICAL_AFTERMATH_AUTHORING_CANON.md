# Destruction And Physical Aftermath Authoring Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Phase-2 owner surface for `Structural Failure Authoring Surface`.
This file governs support graphs, progressive failure, tree fracture, debris persistence, and secondary-collision aftermath.

## Command inventory
- `btn.struct.author_support_graph`
- `btn.struct.bind_failure_policy`
- `btn.struct.inspect_load_path`
- `btn.struct.simulate_cascade_collapse`
- `btn.struct.compare_aftermath_triplet`
- `btn.struct.capture_aftermath_evidence`
- `btn.struct.recover_aftermath_baseline`
- `btn.struct.certify_aftermath_pack`

## Required overlays
- `overlay.struct.support_graph`
- `overlay.struct.load_path`
- `overlay.struct.collapse_cascade`
- `overlay.struct.debris_persistence`
- `overlay.struct.secondary_collision`
- `overlay.struct.failure_board`

## Required inspector fields
- `support_graph_id`
- `critical_support_set`
- `load_path_state`
- `collapse_window`
- `debris_persistence_class`
- `secondary_collision_state`
- `aftermath_baseline_ref`
- `artifact_ref`

## Disabled reasons
- `disable.struct.author_blocked`
- `disable.struct.bind_blocked`
- `disable.struct.inspect_blocked`
- `disable.struct.simulate_blocked`
- `disable.struct.capture_blocked`

## Exact brutal proof slices
1. tree trunk fracture and branch secondary collision.
2. wall or slab progressive collapse.
3. aftermath publication before cleanup.
4. compare, capture, and recovery on the same collapse family.

## Compare and capture law
This surface may use only:
- `compare.struct.aftermath_triplet`
- `capture.struct.aftermath_bundle`

Certification packs allowed from this surface:
- `pack.structure_cascade_aftermath`
- `pack.secondary_collision_aftermath`

## Current posture
`document_gold / doc_closed_impl_open`
