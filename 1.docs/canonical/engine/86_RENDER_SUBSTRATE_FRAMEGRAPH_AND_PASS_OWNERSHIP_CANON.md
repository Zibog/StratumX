# Render Substrate Framegraph and Pass Ownership Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the render substrate, pass graph, render phases, and frame assembly boundaries.

## Truth objects
FrameGraphState, RenderPassPlan, ViewAttachmentSet, FrameResultState

## Runtime phases
frame request accept -> pass graph assemble -> pass execute -> compose result -> publish frame observation

## Diagnostics
missing pass owner, attachment mismatch, graph cycle, unsupported permutation

## Exact editor entrypoints
- frame and render-pipeline command rows in `editor/110` via `editor/90` and `editor/91`

## Required publications
- `obs.frame.*` with frame timing, queue phase, and frame-budget verdict;
- `obs.pipeline.*` with pass ownership, attachment legality, and graph blocker publication;

## Phase-4 brutal proof slices
- `framegraph_legality_and_pass_ownership_visible`;
- `mixed_load_frame_budget_remains_truthful`;

## Old-floor evidence obligations
- one retained old-floor frame bundle showing the actual rung taken;
- one denial sample proving attachment or pass-owner failure is not hidden by fallback;

## Current posture
`document_gold / doc_closed_impl_open / runtime_family_closed_in_docs`
