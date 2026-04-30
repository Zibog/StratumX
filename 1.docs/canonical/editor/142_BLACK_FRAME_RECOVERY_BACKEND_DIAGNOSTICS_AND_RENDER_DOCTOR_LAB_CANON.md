# Black Frame Recovery Backend Diagnostics And Render Doctor Lab Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define editor behavior when the viewport is black, present fails, capture fails, or backend selection is blocked.

## Required operator actions
- explain black frame;
- run render doctor;
- recover swapchain;
- drop feature tier;
- select fallback backend;
- capture failed frame metadata;
- capture recovered frame;
- jump to shader/material/resource/backend blocker.

## Black frame categories
- no backend;
- backend stubbed;
- surface failed;
- swapchain failed;
- no frame plan;
- pass graph empty;
- shader failed;
- pipeline failed;
- resource missing;
- present failed;
- capture failed.

## Required UI posture
The viewport region remains visible.
The failure explanation appears inside the viewport shell.
The diagnostics rail receives the same blocker code.
The operator gets one legal next action.

## Current posture
`document_gold / render_doctor_lab_defined / implementation_open`


---
# V32 Editor Lab Closure: Black Frame Recovery

## Required black-frame view
Classification, frame id, backend class/status, surface/present state, first blocker, affected pass/resource/shader/material if known, recovery action, evidence artifact link.

## Required recovery actions
Run backend doctor, recover swapchain, reset camera, inspect visible set, validate framegraph, open shader failure, open resource residency, apply missing-material fallback, switch to null validation, retry backend policy, capture metadata-only evidence.
