# Graphics Port Null Backend Headless Validation Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define the mandatory null/headless backend used for CI, tests, no-present validation, packet verification, and early graphics-port development.

## Null backend law
The null backend is not a fake renderer. It is a validating backend.

It must:
- accept frame plans;
- validate resource/pass declarations;
- validate handle lifetimes;
- validate feature-tier requests;
- validate capture request shape;
- publish frame outcomes;
- publish no-present verdicts;
- participate in backend policy resolution.

It must not:
- claim pixel output;
- claim real presentation;
- claim GPU timing;
- silently accept invalid resource transitions;
- hide errors because no native API is used.

## Use cases
- CI quality gates;
- headless server validation;
- SDK packet tests;
- tooling route tests;
- frame plan schema tests;
- no raw backend type leak tests;
- editor fallback surface tests.

## Required diagnostics
- invalid pass graph;
- missing target surface;
- illegal capture request;
- resource use before create;
- resource write/read conflict;
- unsupported feature tier;
- no-present expected;
- no-present unexpected.

## Current posture
`document_gold / null_backend_mandatory / implementation_open`


---
# V32 Null Backend Completion

## Null backend responsibilities
Null backend must:
- register as `graphics.backend.null`;
- report T0 feature tier;
- accept `RenderFramePlan`;
- validate frame plan structure;
- validate pass graph acyclicness;
- validate resource references structurally;
- reject present with lawful no-present outcome;
- publish frame outcome packets;
- support metadata-only capture.

## Null backend forbidden behavior
Null must not create a native device, fake rendered screenshots, pretend to present, or silently accept invalid pass/resource/shader references.
