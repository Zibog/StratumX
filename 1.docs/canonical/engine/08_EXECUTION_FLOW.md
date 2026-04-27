# Execution Flow

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Status of This Document

This document is the root execution summary map.

The authoritative normative laws are:

- `constitutions/STRATUMX_EXECUTION_CONSTITUTION.md`
- `constitutions/STRATUMX_WORLD_PROGRESSION_FLOW.md`
- `constitutions/STRATUMX_SIMULATION_TIER_CONSTITUTION.md`
- `constitutions/STRATUMX_CROSS_FAMILY_SOLVE_ORDER.md`

## Canonical Startup Flow

`engine_startup`:

1. loads configuration;
2. selects runtime profile and legal network role;
3. wires lower substrate access;
4. resolves legal prepared resource inputs and runtime-pack inputs;
5. wires runtime resource services against the selected runtime-pack inputs;
6. wires critical simulation families and legal network services;
7. wires service layers where required by profile and cadence;
8. performs legal warmup and launches runtime.

## Canonical Runtime Flow

A runtime profile executes the canonical phase order:

1. ingress;
2. read;
3. compute;
4. resource;
5. authority-sync;
6. stage;
7. apply;
8. egress;
9. diagnostics.

## Canonical Compute Suborder

Inside `compute`, the runtime preserves the authoritative family order frozen by `constitutions/STRATUMX_CROSS_FAMILY_SOLVE_ORDER.md`:

1. `engine_kinetics` primary local solve;
2. `engine_field` primary distributed solve consuming legal same-tick bridge products from `engine_kinetics`;
3. `engine_agents` primary agent solve consuming legal same-tick bridge products from `engine_kinetics` and `engine_field`.

Everything else is deferred through stage and segmented apply.

## Canonical Family Flow

- `engine_kinetics` emits local motion/contact/impact products;
- `engine_field` emits distributed field and region deltas;
- `engine_agents` emits navigation/perception/decision/society/schedule products;
- `engine_stream_control` emits activation, prefetch, and eviction decisions;
- `engine_residency_control` emits residency and pressure products;
- `engine_memory_control` emits allocator, pool, and pressure-control products;
- `engine_transfer_control` emits decode, staging, upload, and release products;
- `engine_net_transport` emits transport events and packet-lane products;
- `engine_net_sync` emits interest, snapshot, delta, and ack products;
- `engine_net_latency` emits prediction, reconciliation, rewind, and correction products;
- `engine_inference` emits normalized model outputs;
- `engine_generation` emits generated products;
- `engine_imaging` emits image synthesis outputs;
- `engine_acoustics` emits acoustic synthesis outputs;
- `engine_content` emits prepared resource and runtime-pack products for startup-time, cook-time, or other resource-facing consumption.
