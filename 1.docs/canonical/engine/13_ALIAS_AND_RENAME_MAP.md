# Alias and Rename Map

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose

This document preserves earlier canonical names and maps them to the current canonical stack.

## Level and Family Renames

| Earlier Name | Canonical Name |
|---|---|
| `L2. Physical Domains` | `L2. Critical Simulation Families` with `engine_kinetics` and `engine_field` |
| `L2. Behavioral Domains` | `L2. Critical Simulation Families` with `engine_agents` |
| `L2. Inference and Dialogue Domains` | `L3.0. Model Systems` |
| `L2. Presentation and Resource Domains` | `L3.1. Synthesis Systems` and `L3.2. Resource Systems` |
| `engine_streaming_runtime` | `L1.5. Runtime Resource Services` |
| `engine_memory_runtime` | `L1.5. Runtime Resource Services` |
| `engine_network` | `L2.5. Network Runtime Services` |
| `L3. Startup` | `L4. Startup` |

## Crate Aliases and Migrations

| Earlier Crate | Canonical Crate / Boundary |
|---|---|
| `engine_collision` | `engine_kinetics` with `Collision` internal boundary |
| `engine_rigidbody` | `engine_kinetics` with `Rigid Body` internal boundary |
| `engine_ballistics` | `engine_kinetics` with `Trajectory` and `Impact` internal boundaries |
| `engine_fluids` | `engine_field` with `Fluid Field` internal boundary |
| `engine_fire` | `engine_field` with `Thermal Field` internal boundary |
| `engine_terrain` | `engine_field` with `Terrain Field` internal boundary |
| `engine_destruction` | `engine_field` with `Structural Field` internal boundary |
| `engine_weather` | `engine_field` with `Atmospheric Field` internal boundary |
| `engine_ai` | `engine_agents` with `Decision` internal boundary |
| `engine_social` | `engine_agents` with `Society` internal boundary |
| `engine_streaming_runtime` | `engine_stream_control` + `engine_residency_control` + `engine_transfer_control` |
| `engine_memory_runtime` | `engine_memory_control` |
| `engine_network` | `engine_net_transport` + `engine_net_sync` + `engine_net_latency` |
| `engine_dialogue` | `engine_generation` for generative boundary; dialogue remains a supported use-case |
| `engine_render` | `engine_imaging` |
| `engine_audio` | `engine_acoustics` |

## Canonical Umbrella Terms

| Umbrella Term | Canonical Scope |
|---|---|
| `lower stack` | `L-1..L0` |
| `runtime family` | `engine_runtime`, `engine_runtime_headless`, `engine_runtime_realtime` |
| `runtime resource services` | `engine_stream_control`, `engine_residency_control`, `engine_memory_control`, `engine_transfer_control` |
| `critical simulation families` | `engine_kinetics`, `engine_field`, `engine_agents` |
| `network runtime services` | `engine_net_transport`, `engine_net_sync`, `engine_net_latency` |
| `service layers` | `L3.0`, `L3.1`, `L3.2` |
| `upper stack` | `L0.5..L4` |
