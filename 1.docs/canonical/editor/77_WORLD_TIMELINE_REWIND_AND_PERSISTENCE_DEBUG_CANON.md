# World Timeline Rewind And Persistence Debug Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Expose legal authoring for save profiles, restore anchors, restore-triplet inspection, restore simulation, compare, capture, recovery, and certification.
It is a production authoring surface, not a review-only room.

## Command inventory
- `btn.timeline.author_save_profile`
- `btn.timeline.bind_restore_anchor`
- `btn.timeline.inspect_restore_triplet`
- `btn.timeline.simulate_restore`
- `btn.timeline.compare_restore_triplet`
- `btn.timeline.capture_restore_evidence`
- `btn.timeline.recover_restore_anchor`
- `btn.timeline.certify_restore_pack`

## Overlay families
- `overlay.timeline.save_profile`
- `overlay.timeline.restore_anchor`
- `overlay.timeline.restore_triplet`
- `overlay.timeline.replay_alignment`
- `overlay.timeline.failure_board`

## Inspector fields
- `save_profile_id`
- `restore_anchor_id`
- `restore_triplet_ref`
- `replay_alignment_state`
- `baseline_ref`
- `artifact_ref`

## Disabled reasons
- `disable.timeline.author_blocked`
- `disable.timeline.bind_blocked`
- `disable.timeline.inspect_blocked`
- `disable.timeline.capture_blocked`

## Compare and capture law
This lab may use only `compare.timeline.restore_triplet` and `capture.timeline.restore_bundle` for `pack.persistence_restore_triplet`.
Any certify, capture, compare, or recover action must lower through the routes named in `editor/110`.

## Post-action focus rules
- success -> editor/105 or editor/109
- retryable failure -> editor/100
- terminal failure -> editor/100 then editor/109
- recovery -> owner lab then compare
