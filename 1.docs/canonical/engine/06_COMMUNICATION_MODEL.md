# Communication Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Canonical Rule

Communication is built on:

- types;
- descriptors;
- views;
- handles;
- snapshots;
- staged outputs;
- synthesized products;
- prepared resource products;
- transport envelopes;
- diagnostics.

## Stack Flow

- lower substrate provides structure;
- world provides authoritative truth;
- material provides shared property truth;
- runtime provides execution windows;
- runtime resource services provide streaming, residency, memory, and transfer control products;
- critical simulation families compute over world truth under explicit tier, cadence, and solve-order law;
- network runtime services ship, filter, predict, reconcile, and validate state under runtime law;
- model systems consume structured context and emit structured outputs;
- synthesis systems emit image and acoustic products;
- resource systems emit prepared resource products and runtime-pack products;
- startup validates, selects, and binds legal runtime-pack inputs into runtime resource services;
- startup assembles, launches the stack, and publishes narrow public bridge export surfaces for the external `L5` boundary.

## Ownership Rule

- world truth ownership stays in `engine_world`;
- global parallel ownership stays in `engine_runtime`;
- runtime resource ownership stays in `engine_stream_control`, `engine_residency_control`, `engine_memory_control`, and `engine_transfer_control`;
- network transport/sync/latency ownership stays in `engine_net_transport`, `engine_net_sync`, and `engine_net_latency`;
- startup owns launch and public bridge export bind, not execution;
- upper families emit staged, prepared, or synthesized products, not arbitrary world truth.

## Immediate vs Deferred Rule

- immediate same-tick family interaction is legal only through explicit bridge surfaces frozen by `constitutions/STRATUMX_CROSS_FAMILY_SOLVE_ORDER.md`;
- direct mutable cross-family calls are illegal;
- deferred outputs cross family boundaries only through staged batches, runtime-governed publication, or segmented apply;
- no service layer may bypass runtime-governed publication to write world truth directly.
