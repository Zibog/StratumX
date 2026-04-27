# STRATUMX_HARDWARE_PROFILE_CONTRACT

## 1. Purpose

This document freezes the operational profile floors and the exact hardware-floor ids used by locked baseline performance proof.
Operational legality may be wider than baseline capture legality. Locked baselines bind only to the exact benchmark floors below.

## 2. Operational Minimums

| Profile | CPU floor | Memory floor | Storage floor | GPU floor |
|---|---|---:|---|---|
| `interactive-60` | >= 4 physical cores / 8 threads | >= 16 GiB | SSD class, >= 350 MiB/s sequential read, <= 1.0 ms p95 for 128 KiB runtime-page read | >= 6 GiB VRAM, legal transfer/storage-buffer/compressed-texture path |
| `listen-host-60` | >= 6 physical cores / 12 threads | >= 24 GiB | SSD class, >= 500 MiB/s sequential read, <= 0.8 ms p95 for 128 KiB runtime-page read | >= 6 GiB VRAM, legal transfer/storage-buffer/compressed-texture path |
| `headless-20` | >= 4 physical cores / 8 threads | >= 16 GiB | SSD class, >= 350 MiB/s sequential read, <= 1.0 ms p95 for 128 KiB runtime-page read | n/a |

## 3. Locked Baseline Benchmark Floors

| Hardware Floor ID | Legal profile(s) | Canonical benchmark floor | Backend / driver binding |
|---|---|---|---|
| `HF-CMN-CPU-01` | `benchmark-common-cpu` | x86-64, 8 physical cores / 16 threads pinned to one CPU package, >= 32 GiB RAM, NVMe SSD >= 2,500 MiB/s sequential read and <= 0.35 ms p95 for 128 KiB runtime-page read | `CPU-ONLY` |
| `HF-INT-60-01` | `interactive-60` | x86-64, 8 physical cores / 16 threads pinned to one CPU package, >= 32 GiB RAM, NVMe SSD >= 2,500 MiB/s sequential read and <= 0.35 ms p95 for 128 KiB runtime-page read, >= 8 GiB VRAM | backend class `VK-RT-01`, driver family `DRV-VK-STABLE-01`, feature tier `FT-RT-01` |
| `HF-LH-60-01` | `listen-host-60` | x86-64, 12 physical cores / 24 threads pinned to one CPU package, >= 32 GiB RAM, NVMe SSD >= 2,500 MiB/s sequential read and <= 0.35 ms p95 for 128 KiB runtime-page read, >= 8 GiB VRAM | backend class `VK-RT-01`, driver family `DRV-VK-STABLE-01`, feature tier `FT-RT-01` |
| `HF-HD-20-01` | `headless-20` | x86-64, 8 physical cores / 16 threads pinned to one CPU package, >= 32 GiB RAM, NVMe SSD >= 2,000 MiB/s sequential read and <= 0.40 ms p95 for 128 KiB runtime-page read | `CPU-ONLY` |

Locked baselines are illegal on any other floor unless a new canonical floor id and capture sheet are added.

## 4. Backend and Driver Identities

| ID | Canonical meaning |
|---|---|
| `VK-RT-01` | canonical Vulkan realtime benchmark backend |
| `DRV-VK-STABLE-01` | canonical stable-release Vulkan driver family used for locked realtime baselines |
| `DRV-POSTURE-STABLE-01` | driver-default stable posture with no forced debug overlays or vendor tuning modes |
| `FT-RT-01` | compressed textures + transfer path + timestamp-query + occlusion-query legal; mesh/task paths optional |
| `PWR-PERF-LOCK-01` | OS performance power mode fixed for the whole run |
| `BOOST-OFF-01` | boost disabled and fixed for the whole run |
| `BOOST-LOCKED-ALLCORE-01` | all-core boost posture fixed and identical across captures |

## 5. Feature Floors

- canonical realtime profiles require a backend capable of legal transfer, storage-buffer, and compressed-texture paths;
- locked GPU baselines are legal only on the backend class, driver family, driver posture, and feature tier frozen above;
- occlusion queries, timestamp queries, pipeline cache, and mesh/task paths remain optional accelerators only at runtime, but any locked baseline row that depends on them must capture their exact availability;
- lack of an optional accelerator does not invalidate runtime profile legality when the canonical fallback path remains legal.

## 6. Benchmark Binding

Locked baseline capture and regression proof are interpreted against the hardware floor ids above, the backend/driver binding above, `STRATUMX_BENCHMARK_PROTOCOL.md`, and `STRATUMX_BASELINE_CAPTURE_SHEETS.md`.

The engine-only registered gold package claim is sealed only for the registered benchmark floors, backend ids, capture-sheet ids, and capture-result ids above, not for arbitrary legal realtime backends.
