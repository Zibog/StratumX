# Texture Streaming Residency MIP and Sampler Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own texture residency, mip selection, sampler legality, and pressure response.

## Truth objects
TextureResidencyState, MipSelectionState, SamplerPolicyState

## Runtime phases
request page -> evaluate residency -> select mip -> bind sampler -> publish residency result

## Diagnostics
fallback mip used, residency denied, budget pressure degrade, sampler mismatch

## Exact editor entrypoints
- residency command rows in `editor/110` via `editor/93`

## Required publications
- `obs.residency.*` with mip choice, sampler policy, residency rung, and budget-pressure reason;

## Phase-4 brutal proof slices
- `4k_residency_degrades_without_silent_lie`;
- `mip_and_sampler_fallback_publish_same_denial_chain`;

## Old-floor evidence obligations
- one old-floor residency bundle with typed rung drop;
- one denial sample proving residency refusal remains visible to the operator;

## Current posture
`document_gold / doc_closed_impl_open / runtime_family_closed_in_docs`
