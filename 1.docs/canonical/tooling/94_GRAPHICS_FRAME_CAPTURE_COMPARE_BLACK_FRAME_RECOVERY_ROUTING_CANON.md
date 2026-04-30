# Graphics Frame Capture Compare Black Frame Recovery Routing Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define tooling routes that capture frames, compare visual evidence, explain black frames, and recover render failures.

## Required routes
- `route.graphics.capture_frame`
- `route.graphics.capture_no_present`
- `route.graphics.compare_frame`
- `route.graphics.explain_black_frame`
- `route.graphics.recover_swapchain`
- `route.graphics.recover_feature_tier`
- `route.graphics.capture_recovered_frame`

## Required artifacts
- captured image or no-present proof;
- backend caps dump;
- frame plan summary;
- pass/resource summary;
- shader/pipeline summary;
- black-frame reason chain;
- recovery action log;
- comparison digest.

## Focus return
After a failure, tooling must return editor focus to:
- backend lab for backend failures;
- shader/material lab for shader/pipeline failures;
- resource/residency lab for missing resource failures;
- viewport shell for surface/swapchain failures.

## Current posture
`document_gold / frame_capture_recovery_routes_defined / implementation_open`


---
# V32 Tooling Closure: Capture/Black-Frame Routes

## Required routes
- `route.graphics.capture_frame`
- `route.graphics.capture_metadata_only`
- `route.graphics.compare_frame`
- `route.graphics.black_frame_triage`
- `route.graphics.recover_swapchain`
- `route.graphics.recover_backend`
- `route.graphics.evidence_bundle`

## Routing table
No surface -> backend probe. Acquire failed -> recover swapchain. No visible set/all culled -> camera/culling diagnostics. No pass submitted -> framegraph validation. Shader/pipeline failed -> shader cook diagnostics. Resource/material missing -> residency/material lab. Present failed -> backend recovery/fallback. Capture failed -> metadata-only evidence.
