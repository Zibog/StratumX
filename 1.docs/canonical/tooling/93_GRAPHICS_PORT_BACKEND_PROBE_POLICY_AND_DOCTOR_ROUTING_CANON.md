# Graphics Port Backend Probe Policy And Doctor Routing Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define tooling routes for backend probing, policy resolution, and render doctor output.

## Required routes
- `route.graphics.backend_probe`
- `route.graphics.backend_policy_resolve`
- `route.graphics.backend_caps_dump`
- `route.graphics.doctor`
- `route.graphics.backend_force_attempt`
- `route.graphics.backend_fallback_explain`

## Route stages
1. collect compiled backend descriptors;
2. probe runtime availability;
3. resolve policy;
4. publish caps/status packets;
5. expose fallback chain;
6. emit doctor artifact;
7. return focus target to editor backend lab.

## Blocker families
- backend not built;
- backend stubbed;
- platform illegal;
- SDK unavailable;
- loader missing;
- device unavailable;
- surface unavailable;
- capture unsupported.

## Current posture
`document_gold / backend_doctor_routes_defined / implementation_open`


---
# V32 Tooling Closure: Backend Doctor Routes

## Required routes
- `route.graphics.backend_probe`
- `route.graphics.policy_resolve`
- `route.graphics.doctor`
- `route.graphics.stub_audit`
- `route.graphics.feature_tier_audit`

## Doctor output
Backend list, selected backend, unavailable backend reasons, stub blockers, shader targets, present/capture support, feature tier, first disabled fast path, and recommended next action.
