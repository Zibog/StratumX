# Animation Intent Pose Contact Solve and Skinning Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own animation intent resolution, pose generation, contact solve, and final skinning publication.

## Truth objects
MotionIntentState, PoseState, ContactSolveState, SkinningState

## Runtime phases
intent resolve -> pose blend -> contact solve -> skin -> publish animation result

## Diagnostics
solver fallback, contact target lost, skeleton mismatch

## Exact editor entrypoints
- animation-runtime command rows in `editor/110` via `editor/96`

## Required publications
- `obs.anim_runtime.*` with intent, pose, contact-solve verdict, skinning state, and first fallback rung;

## Phase-4 brutal proof slices
- `door_handle_micro_motion_contact_solve_without_canned_middle`;
- `runtime_skinning_keeps_contact_target_truthful_after_recover`;

## Old-floor evidence obligations
- one retained old-floor bundle proving solver fallback rung stays visible;
- one compare digest proving baseline/current/recovered solve truth;

## Current posture
`document_gold / doc_closed_impl_open / runtime_family_closed_in_docs`
