# Graphics Port Backend Policy And Viewport Device Lab Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define the editor lab where the operator can see and control StratumX graphics backend policy without touching native backend implementation.

## Required controls
- backend policy selector: auto, forced, benchmark locked, headless validation;
- backend list: null, Vulkan, Direct3D 12, Metal, platform native;
- force backend button;
- run backend doctor button;
- dump backend caps button;
- show fallback chain button;
- recover backend button.

## Required badges
- selected backend;
- backend status;
- feature tier;
- shader target set;
- present path;
- capture readiness;
- first blocker;
- fallback backend.

## Disabled reasons
- backend not built;
- backend stubbed;
- platform illegal;
- SDK unavailable;
- native loader missing;
- no compatible device;
- no present surface;
- forced backend failed.

## Law
The editor may select policy and show diagnostics. It may not import native API types or call backend-specific code directly.

## Current posture
`document_gold / backend_policy_lab_defined / implementation_open`


---
# V32 Editor Lab Closure: Backend Policy

## Required sections
Backend policy selector, backend list with statuses, selected backend caps, feature tier ladder, optional feature verdicts, present surface state, fallback chain, first blocker details, and render doctor output.

## Required actions
Select auto policy, force backend, lock benchmark backend, switch to null/headless validation, run backend probe, run render doctor, dump backend caps, retry backend resolution, open blocker evidence.
