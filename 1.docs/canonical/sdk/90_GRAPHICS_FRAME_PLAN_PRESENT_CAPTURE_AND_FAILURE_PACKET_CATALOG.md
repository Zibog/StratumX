# Graphics Frame Plan Present Capture And Failure Packet Catalog

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define packets that describe frame plans, present outcomes, capture outcomes, and graphics failure explanations.

## Packet families
| Packet | Purpose |
|---|---|
| `packet.graphics.frame_plan_summary.v1` | frame id, view id, pass count, resource count, capture intent |
| `packet.graphics.prepared_frame.v1` | backend preparation result and predicted blockers |
| `packet.graphics.present_result.v1` | swapchain/no-present outcome |
| `packet.graphics.capture_result.v1` | retained image/no-present proof and metadata |
| `packet.graphics.black_frame_reason.v1` | exact first blocker and reason chain |
| `packet.graphics.recovery_result.v1` | recovery route and final outcome |

## Failure fields
- family;
- backend class;
- stage;
- first blocker;
- recoverability;
- operator label;
- developer detail;
- associated resource id;
- associated shader/pipeline id;
- associated pass id.

## Retention law
Captured frame packets must be retained with the image/no-present proof and backend caps used for the frame.

## Current posture
`document_gold / frame_present_capture_packets_defined / implementation_open`


---
# V32 SDK Closure: Frame/Present/Capture Packets

## Required packets
| Packet | Purpose |
|---|---|
| `packet.graphics.frame_scope.v1` | frame id, viewport/surface, policy |
| `packet.graphics.frame_plan_summary.v1` | pass/resource/material/shader summary |
| `packet.graphics.present_outcome.v1` | present/no-present/failure result |
| `packet.graphics.capture_outcome.v1` | capture artifact metadata |
| `packet.graphics.render_failure.v1` | exact render failure and recovery hint |
| `packet.graphics.black_frame.v1` | black-frame classification |
| `packet.graphics.frame_evidence_bundle.v1` | retained proof bundle |

## Outcome states
Present: `presented`, `no_present_headless`, `skipped_resize`, `swapchain_recreated`, `surface_lost`, `present_failed`, `backend_failed`.

Capture: `captured_image`, `captured_metadata_only`, `capture_degraded`, `capture_failed`, `capture_not_supported`.
