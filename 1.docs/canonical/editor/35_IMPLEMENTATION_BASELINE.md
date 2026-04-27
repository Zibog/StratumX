# Implementation Baseline

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Baseline expectations
- one canonical Rust-native editor host
- one shell and one primary viewport above lower-stack authority
- one honest engine-to-viewport frame path
- viewport-centric interaction
- boot-to-viewport level-space readiness
- suite and service registration
- activation-bounded execution
- request/view separation
- index-backed large-world and content browsing
- explicit runtime bridge, validation, build, and release surfaces

## Physical implementation expectations
- shell and surface state split from heavy service jobs
- view models fed by immutable snapshots, bounded streams, indices, and projections from lower stack
- tool contexts and suite state kept narrow and local
- background jobs isolated from focused interaction routing
- one legal startup-world and world-open path from host to viewport
- plugins mounted through explicit registration points only
- terrain and sky/environment presented through the same world identity
- runtime-entry and return-to-authoring routed through explicit result-bearing contracts

## Spine-first implementation law
Before broad chrome expansion, the implementation must close this exact spine:
- host
- shell
- world-open and restore
- primary viewport
- outliner
- inspector
- content browser
- terrain suite
- environment suite
- diagnostics rail
- play/simulate entry
- return to authoring

## First product-visible closure target
The first closure target is not broad feature count.
It is one honest world result:
- boot the editor;
- open a legal reference world;
- show real terrain in the primary viewport;
- show the legal sky/environment result in the same viewport;
- expose diagnostics strong enough to explain the result;
- enter play/simulate and walk the same world;
- return to authoring cleanly.
