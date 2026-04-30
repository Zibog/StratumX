# Viewport Frame Capture Black Frame And Render Diagnostics Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define diagnostics and recovery for first rendering, black frames, failed presents, capture gaps, and viewport evidence.

## Black-frame law
A black frame must never be treated as unknown.

Required blocker families:
- no backend selected;
- backend unavailable;
- surface creation failed;
- swapchain unavailable;
- image acquire failed;
- no frame plan submitted;
- pass graph empty;
- shader compile failed;
- pipeline creation failed;
- resource missing;
- binding layout mismatch;
- barrier/layout transition invalid;
- present failed;
- capture resolve failed.

## Frame capture artifact
A render capture artifact contains:
- image or no-present proof;
- backend class/status;
- policy;
- feature tier;
- shader target set;
- active passes;
- visible overlays;
- disabled fast paths;
- first blocker;
- build id;
- world/view id.

## Recovery law
Recovery may:
- recreate swapchain;
- drop to lower feature tier;
- fall back to null/no-present;
- disable optional accelerator;
- use fallback shader variant;
- reload missing resource placeholder.

Recovery may not:
- hide failure;
- claim original success;
- bypass capture metadata;
- mutate material truth to make render succeed.

## Current posture
`document_gold / render_diagnostics_defined / implementation_open`


---
# V32 Diagnostics Closure

## Required captured frame metadata
- image artifact when available;
- frame id;
- backend class/status/caps;
- policy resolution;
- surface/present state;
- framegraph node list;
- resource residency summary;
- shader variant summary;
- material set summary;
- output color metadata;
- fallback/degradation rungs;
- timing summary when available;
- black-frame classification when relevant.

## Required diagnostics
Backend selected, present state, frame-in-flight state, framegraph state, shader compile state, pipeline cache state, resource residency state, material fallback state, output color state, capture state, and first blocker.
