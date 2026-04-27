# Camera Sensor Exposure Tonemap and Post FX Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own camera sensor model, exposure adaptation, tone mapping, and legal post-processing chain.

## Truth objects
CameraSensorState, ExposureState, TonemapState, PostFxState

## Runtime phases
camera resolve -> exposure -> tonemap -> post chain -> publish presentable frame

## Diagnostics
auto exposure clamped, tonemap fallback, post effect disabled

## Exact editor entrypoints
- sky/atmosphere presentation command rows in `editor/110` via `editor/94` whenever exposure/post participates in a retained presentation proof

## Required publications
- `obs.camera.*` with exposure state, tonemap family, post-chain verdict, and first disable reason;

## Phase-4 brutal proof slices
- `exposure_and_post_do_not_fake_lighting_truth`;
- `post_chain_disable_reason_remains_visible`;

## Old-floor evidence obligations
- one denial sample proving post disable reason is published;
- one retained bundle proving exposure/post stayed aligned with the underlying light/media truth;

## Current posture
`document_gold / doc_closed_impl_open / runtime_family_closed_in_docs`
