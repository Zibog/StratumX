# STRATUMX_BASELINE_CAPTURE_RESULTS

## 1. Purpose

This document freezes the package-contained capture-result artifacts that back the locked baseline table.
A locked baseline is not package-proven unless it resolves to both a registered capture sheet and a registered capture-result artifact here.

## 2. Rule

Each capture-result artifact binds:
- one capture-sheet id;
- one hardware floor id;
- one backend id;
- one driver family/posture pair;
- one power/turbo posture;
- one compiler/target tuple;
- one or more exact fixture ids; and
- one or more exact baseline ids captured under that exact posture.

A baseline row is illegal when its capture-sheet id exists but no capture-result artifact is registered here for the same posture.
No capture-result row in this registry may rely on wildcard, family-only, or prose-only baseline binding.

## 3. Capture Result Artifacts

| Capture Result ID | Capture Sheet ID | Hardware Floor ID | Backend ID | Driver Family ID | Driver Posture ID | Power Mode ID | Turbo/Boost ID | Compiler / Target Tuple | Bound Fixture IDs | Bound baseline ids | Stack Marker |
|---|---|---|---|---|---|---|---|---|---|---|---|
| `CR-CMN-CPU-01` | `CS-CMN-CPU-01` | `HF-CMN-CPU-01` | `CPU-ONLY` | `DRV-N/A` | `DRV-N/A` | `PWR-PERF-LOCK-01` | `BOOST-OFF-01` | `rustc 1.94.0 / x86_64-pc-windows-msvc` | `lower-stack-traversal-a`, `runtime-kernel-a`, `streaming-locality-a`, `world-apply-a`, `replay-determinism-a`, `service-normalization-a`, `content-transform-a` | `BL-LS-CORE-01`, `BL-LS-ID-01`, `BL-LS-HANDLE-01`, `BL-LS-LAYOUT-01`, `BL-LS-ACCESS-01`, `BL-LS-REG-01`, `BL-LS-QUERY-01`, `BL-LS-ECS-01`, `BL-LS-SPATIAL-01`, `BL-LS-REGION-01`, `BL-STORAGE-MUT-01`, `BL-LS-MAT-01`, `BL-RT-CONT-01`, `BL-SVC-MEM-01`, `BL-MODEL-INF-01`, `BL-MODEL-GEN-01`, `BL-CONTENT-01`, `BL-REPLAY-CMP-01` | `SX-CANON/1.0.24/STACK-v30` |
| `CR-INT-60-CPU-01` | `CS-INT-60-CPU-01` | `HF-INT-60-01` | `CPU-ONLY` | `DRV-N/A` | `DRV-N/A` | `PWR-PERF-LOCK-01` | `BOOST-LOCKED-ALLCORE-01` | `rustc 1.94.0 / x86_64-pc-windows-msvc` | `runtime-kernel-a`, `streaming-locality-a`, `acoustics-streaming-a`, `startup-assembly-a` | `BL-RT-EMPTY-01`, `BL-RT-MIN-01`, `BL-RT-ORCH-01`, `BL-SVC-STREAM-INT-01`, `BL-SVC-RESID-INT-01`, `BL-AUDIO-INT-01`, `BL-STARTUP-01` | `SX-CANON/1.0.24/STACK-v30` |
| `CR-INT-60-VK-01` | `CS-INT-60-VK-01` | `HF-INT-60-01` | `VK-RT-01` | `DRV-VK-STABLE-01` | `DRV-POSTURE-STABLE-01` | `PWR-PERF-LOCK-01` | `BOOST-LOCKED-ALLCORE-01` | `rustc 1.94.0 / x86_64-pc-windows-msvc` | `mixed-pressure-realtime-a`, `visibility-realtime-a`, `streaming-locality-a` | `BL-REALTIME-INT-01`, `BL-MIXED-REALTIME-INT-01`, `BL-IMG-INT-01`, `BL-SVC-XFER-INT-01` | `SX-CANON/1.0.24/STACK-v30` |
| `CR-LH-60-CPU-01` | `CS-LH-60-CPU-01` | `HF-LH-60-01` | `CPU-ONLY` | `DRV-N/A` | `DRV-N/A` | `PWR-PERF-LOCK-01` | `BOOST-LOCKED-ALLCORE-01` | `rustc 1.94.0 / x86_64-pc-windows-msvc` | `streaming-locality-a`, `network-listen-host-a`, `acoustics-streaming-a`, `destruction-heavy-a`, `agent-cycle-a` | `BL-SVC-STREAM-LH-01`, `BL-SVC-RESID-LH-01`, `BL-SVC-MEM-LH-01`, `BL-NET-TX-LH-01`, `BL-NET-SYNC-LH-01`, `BL-NET-LAT-LH-01`, `BL-KIN-01`, `BL-FIELD-01`, `BL-AGENT-01`, `BL-AUDIO-LH-01` | `SX-CANON/1.0.24/STACK-v30` |
| `CR-LH-60-VK-01` | `CS-LH-60-VK-01` | `HF-LH-60-01` | `VK-RT-01` | `DRV-VK-STABLE-01` | `DRV-POSTURE-STABLE-01` | `PWR-PERF-LOCK-01` | `BOOST-LOCKED-ALLCORE-01` | `rustc 1.94.0 / x86_64-pc-windows-msvc` | `mixed-pressure-realtime-a`, `visibility-realtime-a`, `streaming-locality-a` | `BL-REALTIME-LH-01`, `BL-MIXED-REALTIME-LH-01`, `BL-IMG-LH-01`, `BL-SVC-XFER-LH-01` | `SX-CANON/1.0.24/STACK-v30` |
| `CR-HD-20-CPU-01` | `CS-HD-20-CPU-01` | `HF-HD-20-01` | `CPU-ONLY` | `DRV-N/A` | `DRV-N/A` | `PWR-PERF-LOCK-01` | `BOOST-OFF-01` | `rustc 1.94.0 / x86_64-pc-windows-msvc` | `mixed-pressure-headless-a`, `network-headless-a`, `world-apply-a`, `streaming-locality-a` | `BL-HEADLESS-01`, `BL-MIXED-HEADLESS-01`, `BL-NET-TX-HD-01`, `BL-NET-SYNC-HD-01`, `BL-NET-LAT-HD-01`, `BL-SVC-STREAM-HD-01`, `BL-SVC-RESID-HD-01`, `BL-SVC-MEM-HD-01`, `BL-SVC-XFER-HD-01` | `SX-CANON/1.0.24/STACK-v30` |

## 4. Measured Baseline Records

| Baseline ID | Capture Result ID | Fixture ID | Measured record | Run window | Stack Marker |
|---|---|---|---|---|---|
| `BL-LS-CORE-01` | `CR-CMN-CPU-01` | `lower-stack-traversal-a` | p95 = 0.041 ms per 100,000 ops | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-LS-ID-01` | `CR-CMN-CPU-01` | `lower-stack-traversal-a` | p95 = 0.395 ms per 10,000 ops | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-LS-HANDLE-01` | `CR-CMN-CPU-01` | `lower-stack-traversal-a` | p95 = 0.263 ms per 10,000 ops | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-LS-LAYOUT-01` | `CR-CMN-CPU-01` | `lower-stack-traversal-a` | p95 = 0.158 ms per 10,000 ops | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-LS-ACCESS-01` | `CR-CMN-CPU-01` | `lower-stack-traversal-a` | p95 = 0.172 ms per 10,000 ops | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-LS-REG-01` | `CR-CMN-CPU-01` | `lower-stack-traversal-a` | p95 = 0.291 ms per 10,000 ops | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-LS-QUERY-01` | `CR-CMN-CPU-01` | `lower-stack-traversal-a` | 14.56 M traversed items/s | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-LS-ECS-01` | `CR-CMN-CPU-01` | `lower-stack-traversal-a` | p95 = 0.169 ms per 10,000 facade ops | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-LS-SPATIAL-01` | `CR-CMN-CPU-01` | `lower-stack-traversal-a` | p95 = 0.0564 ms per 100,000 reads | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-LS-REGION-01` | `CR-CMN-CPU-01` | `lower-stack-traversal-a` | p95 = 0.132 ms per 10,000 ops | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-STORAGE-MUT-01` | `CR-CMN-CPU-01` | `world-apply-a` | p95 = 0.246 ms per 10,000 staged records | 24 progression runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-LS-MAT-01` | `CR-CMN-CPU-01` | `lower-stack-traversal-a` | p95 = 0.047 ms per 100,000 lookups | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-RT-EMPTY-01` | `CR-INT-60-CPU-01` | `runtime-kernel-a` | p95 = 11.28 us | 50 microbench runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-RT-MIN-01` | `CR-INT-60-CPU-01` | `runtime-kernel-a` | p95 = 20.68 us | 50 microbench runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-RT-ORCH-01` | `CR-INT-60-CPU-01` | `runtime-kernel-a` | p95 = 0.451 ms | 50 microbench runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-RT-CONT-01` | `CR-CMN-CPU-01` | `runtime-kernel-a` | blocked-worker ratio = 0.34% | 50 microbench runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-SVC-STREAM-INT-01` | `CR-INT-60-CPU-01` | `streaming-locality-a` | p95 = 0.282 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-SVC-RESID-INT-01` | `CR-INT-60-CPU-01` | `streaming-locality-a` | p95 = 0.207 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-SVC-MEM-01` | `CR-CMN-CPU-01` | `streaming-locality-a` | p95 = 0.113 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-SVC-XFER-INT-01` | `CR-INT-60-VK-01` | `streaming-locality-a` | p95 = 0.705 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-SVC-STREAM-LH-01` | `CR-LH-60-CPU-01` | `streaming-locality-a` | p95 = 0.329 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-SVC-RESID-LH-01` | `CR-LH-60-CPU-01` | `streaming-locality-a` | p95 = 0.263 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-SVC-MEM-LH-01` | `CR-LH-60-CPU-01` | `streaming-locality-a` | p95 = 0.132 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-SVC-XFER-LH-01` | `CR-LH-60-VK-01` | `streaming-locality-a` | p95 = 0.846 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-SVC-STREAM-HD-01` | `CR-HD-20-CPU-01` | `streaming-locality-a` | p95 = 0.395 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-SVC-RESID-HD-01` | `CR-HD-20-CPU-01` | `streaming-locality-a` | p95 = 0.301 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-SVC-MEM-HD-01` | `CR-HD-20-CPU-01` | `streaming-locality-a` | p95 = 0.150 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-SVC-XFER-HD-01` | `CR-HD-20-CPU-01` | `streaming-locality-a` | p95 = 0.423 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-HEADLESS-01` | `CR-HD-20-CPU-01` | `world-apply-a` | parallel efficiency = 0.91 | 24 progression runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-REALTIME-INT-01` | `CR-INT-60-VK-01` | `mixed-pressure-realtime-a` | frame jitter = 1.16% | 30 integrated runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-REALTIME-LH-01` | `CR-LH-60-VK-01` | `mixed-pressure-realtime-a` | frame jitter = 1.44% | 30 integrated runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-MIXED-REALTIME-INT-01` | `CR-INT-60-VK-01` | `mixed-pressure-realtime-a` | p95 frame time = 13.50 ms; max tracked class = 72% of hard ceiling | 30 integrated runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-MIXED-REALTIME-LH-01` | `CR-LH-60-VK-01` | `mixed-pressure-realtime-a` | p95 frame time = 13.50 ms; max tracked class = 68% of hard ceiling | 30 integrated runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-MIXED-HEADLESS-01` | `CR-HD-20-CPU-01` | `mixed-pressure-headless-a` | p95 tick time = 17.50 ms; max tracked class = 71% of hard ceiling | 30 integrated runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-NET-TX-LH-01` | `CR-LH-60-CPU-01` | `network-listen-host-a` | p95 = 0.179 ms per peer | 40 peer-window runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-NET-SYNC-LH-01` | `CR-LH-60-CPU-01` | `network-listen-host-a` | p95 = 0.263 ms per peer | 40 peer-window runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-NET-LAT-LH-01` | `CR-LH-60-CPU-01` | `network-listen-host-a` | p95 = 0.150 ms per peer | 40 peer-window runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-NET-TX-HD-01` | `CR-HD-20-CPU-01` | `network-headless-a` | p95 = 0.197 ms per peer | 40 peer-window runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-NET-SYNC-HD-01` | `CR-HD-20-CPU-01` | `network-headless-a` | p95 = 0.282 ms per peer | 40 peer-window runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-NET-LAT-HD-01` | `CR-HD-20-CPU-01` | `network-headless-a` | p95 = 0.166 ms per peer | 40 peer-window runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-KIN-01` | `CR-LH-60-CPU-01` | `destruction-heavy-a` | p95 = 2.21 ms | 24 progression runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-FIELD-01` | `CR-LH-60-CPU-01` | `destruction-heavy-a` | p95 = 2.57 ms | 24 progression runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-AGENT-01` | `CR-LH-60-CPU-01` | `agent-cycle-a` | p95 = 1.97 ms | 24 progression runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-IMG-INT-01` | `CR-INT-60-VK-01` | `visibility-realtime-a` | p95 = 0.854 ms | 30 integrated runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-IMG-LH-01` | `CR-LH-60-VK-01` | `visibility-realtime-a` | p95 = 0.901 ms | 30 integrated runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-AUDIO-INT-01` | `CR-INT-60-CPU-01` | `acoustics-streaming-a` | p95 = 0.612 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-AUDIO-LH-01` | `CR-LH-60-CPU-01` | `acoustics-streaming-a` | p95 = 0.689 ms | 36 locality runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-MODEL-INF-01` | `CR-CMN-CPU-01` | `service-normalization-a` | p95 = 0.235 ms | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-MODEL-GEN-01` | `CR-CMN-CPU-01` | `service-normalization-a` | p95 = 0.282 ms | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-CONTENT-01` | `CR-CMN-CPU-01` | `content-transform-a` | p95 = 3.18 ms per canonical page-transform unit | 24 transform runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-STARTUP-01` | `CR-INT-60-CPU-01` | `startup-assembly-a` | p95 = 169.2 ms warm start | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
| `BL-REPLAY-CMP-01` | `CR-CMN-CPU-01` | `replay-determinism-a` | p95 = 1.31 ms | 32 canonical runs | `SX-CANON/1.0.24/STACK-v30` |
