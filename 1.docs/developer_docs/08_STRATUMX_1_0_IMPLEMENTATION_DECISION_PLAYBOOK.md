# StratumX 1.0 Implementation Decision Playbook

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **developer implementation playbook**.

## Purpose

This playbook tells an implementation agent which decisions are already made and how to implement them without re-arguing the architecture.

## Implementation order

1. Stabilize workspace and quality gates.
2. Implement Native Graphics Port core/null/stubs/first real backend.
3. Implement source/cooked asset pipeline.
4. Implement hybrid material profile runtime.
5. Implement showable frame route.
6. Implement world/cell/chunk placement.
7. Implement import/cook/preview/capture loop.
8. Add physical material preview channels.
9. Add audio material event preview.
10. Keep netcode at foundation boundaries unless the phase explicitly targets 1.x.

## What to code for hybrid materials

Create:

- material family registry;
- parameter schema validator;
- texture channel binding validator;
- shader variant key builder;
- material preview route;
- fallback material generator;
- optional graph compile target interfaces only after profiles work.

Do not create:

- free-form graph runtime first;
- shader-specific material truth;
- backend-specific material definitions;
- editor-only material truth.

## What to code for asset pipeline

Create:

- source asset descriptor;
- importer registry;
- support class enum P0/P1/P2/P3/Q;
- Blender bridge adapter;
- 3ds Max bridge adapter;
- exchange importer adapters;
- cooked package descriptors;
- conversion reports;
- import blocker codes;
- reimport identity preservation.

Do not create:

- runtime direct `.blend` or `.max` parser;
- source-file dependency in runtime;
- silent material conversion;
- import success without cooked package or blocker.

## What to code for world

Create:

- world id;
- region/sector/cell/chunk ids;
- local coordinate conversion;
- world entity identity;
- component storage;
- authoring projection;
- cooked world chunk package.

## What to code for scheduler

Create phase scheduler with explicit phase registration. Systems submit work to phases. No domain owns global threading.

## What to code for audio

Create event descriptors, material sound rows, variation sets, bank manifest, bus routes, audition route, no-sound diagnostics. Keep device backend replaceable.

## What to code for netcode foundation

Create packet types and authority modes now. Do not pretend production multiplayer is finished. Ensure world/entity/material/assets can be replicated later by stable ids and deltas.

## Completion checklist

- All new implementation maps to a canonical decision.
- Every user-visible system has command, packet, tutorial, troubleshooting.
- Every source asset produces cooked package or exact blocker.
- Every placeholder reports stub/not implemented, never success.
