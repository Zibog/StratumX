# Implementation Handoff

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines the implementation handoff for the SDK bridge package.

## Build target
Implement `L5` as a compact, replay-safe boundary layer that can feed a large editor/tooling stack without growing editor semantics inside the bridge.

## Non-negotiable outcomes
- ordered write ingress;
- immutable read publication;
- explicit compatibility and legality surfaces;
- opaque handles/refs/artifact-refs only;
- no hidden mutable mirror of engine truth;
- no editor-shaped payload structs on bridge hot paths.

## What upper layers are allowed to expect
`tooling/` and `editor/` may expect:
- stable ids and opaque refs;
- versioned immutable snapshots;
- cursor-safe observation and metric batches;
- legality verdicts and transport policies;
- artifact refs that do not transfer artifact ownership.

They may not expect:
- fully assembled authoring objects;
- prefab or package semantics;
- panel-ready widget payloads;
- assistant or planning state.

## Build order
1. define bridge ids/handles/refs;
2. implement immutable snapshot and batch publication;
3. implement legality/compatibility tables;
4. align invalidation/epoch rules with `L6` consumers;
5. pressure-test locality and allocation posture.
