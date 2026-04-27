# STRATUMX_ENABLEMENT_LEGALITY_MATRIX

## 1. Purpose

This document defines which legal units are always-on, legally optional, or illegal to disable.

## 2. Scope

This matrix covers:
- lower stack;
- shared world property substrate;
- runtime family;
- runtime resource services;
- critical simulation families;
- network runtime services;
- service layers;
- startup.

## 3. Matrix

| Unit | Unit Class | Always-On | Legal to Disable | Canonical Note |
|---|---|---:|---:|---|
| lower stack | mandatory unit | yes | no | Foundation through world truth are mandatory in every legal engine assembly. |
| engine_material | mandatory unit | yes | no | Shared world property substrate is mandatory for canonical upper execution. |
| runtime family | mandatory unit | yes | no | One runtime profile must exist for one legal world instance. |
| engine_runtime_headless | profile unit | no | yes | Optional as a profile implementation when not selected. |
| engine_runtime_realtime | profile unit | no | yes | Optional as a profile implementation when not selected. |
| engine_stream_control | mandatory resource service | yes | no | Runtime-owned stream activation and IO control are mandatory. |
| engine_residency_control | mandatory resource service | yes | no | Runtime-owned residency visibility and budgets are mandatory. |
| engine_memory_control | mandatory resource service | yes | no | Runtime-owned allocator and lifetime control are mandatory. |
| engine_transfer_control | conditional resource service | no | yes | Legal to disable only when the selected assembly has no runtime decode/upload/readback target. |
| engine_kinetics | mandatory family | yes | no | Critical simulation family. |
| engine_field | mandatory family | yes | no | Critical simulation family. |
| engine_agents | mandatory family | yes | no | Critical simulation family. |
| engine_net_transport | optional network service | no | yes | Legal to disable in offline assemblies. Mandatory when a network role is selected. |
| engine_net_sync | optional network service | no | yes | Legal to disable in offline assemblies. Mandatory when replicated authority is selected. |
| engine_net_latency | optional network service | no | yes | Legal to disable in offline assemblies. Mandatory when prediction/rewind-capable network roles are selected. |
| engine_inference | optional service layer | no | yes | Legal to disable if no inference-driven workload is assembled. |
| engine_generation | optional service layer | no | yes | Legal to disable if no generation workload is assembled. |
| engine_imaging | optional service layer | no | yes | Legal to disable in headless or non-image-producing assemblies. |
| engine_acoustics | optional service layer | no | yes | Legal to disable in silent or non-acoustic assemblies. |
| engine_content | optional service layer | no | yes | Legal to disable only when startup consumes pre-prepared resource inputs and runtime-pack inputs from outside `engine_content`. |
| engine_startup | mandatory unit | yes | no | Assembly-time mandatory owner. Not a runtime-resident unit. |

## 4. Canonical Rule

Only units marked legal to disable may be absent from a legal enablement set.
