# Graphics Port Backend Registry Policy And Capability Publication Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Own the runtime registry of available graphics backends and the publication of backend capabilities to SDK, tooling, editor, and diagnostics.

## Registry contents
Every backend descriptor contains:
- backend class;
- backend family;
- build availability;
- platform availability;
- implementation status;
- shader target set;
- supported feature tiers;
- optional accelerators;
- presentation support;
- capture support;
- known blockers;
- preferred policy conditions.

## Status ladder
`not_built -> built_stub -> built_available -> selected -> active -> failed -> recovered -> shutdown`

## Capability publication
A backend must publish:
- limits;
- supported formats;
- supported present modes;
- supported sample counts;
- maximum texture dimensions;
- buffer alignment rules;
- upload granularity;
- timestamp support;
- debug marker support;
- capture support;
- optional accelerator list.

## Policy resolver phases
1. collect compiled backend descriptors;
2. filter by platform legality;
3. filter by requested policy;
4. probe runtime availability;
5. rank candidates;
6. select candidate;
7. compute fallback chain;
8. publish resolution packet;
9. allow editor/tooling override if legal;
10. retain selected result for capture.

## Failure publication
If no backend is usable:
- shell may still boot;
- null/no-present backend is attempted if allowed;
- editor viewport shows failure surface;
- tooling doctor emits blocker rows;
- SDK publishes terminal backend selection failure.

## Current posture
`document_gold / registry_policy_defined / implementation_open`


---
# V32 Backend Registry Completion

## Backend descriptor fields
Each backend descriptor must include:
- backend class;
- backend family;
- build availability;
- platform legality;
- status;
- first blocker;
- shader targets;
- supported feature tier;
- optional feature verdicts;
- presentation caps;
- capture caps;
- limits;
- diagnostic hooks.

## Required tests
Quality must verify:
- all required backend families are registered;
- null backend is always available in headless profile;
- stubs never report successful present;
- Vulkan is not selected by default on every platform;
- forced unavailable backend yields first blocker;
- benchmark locked backend refuses silent fallback;
- backend caps serialize to SDK packet shape.
