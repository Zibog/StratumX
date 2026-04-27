# STRATUMX_BASELINE_CAPTURE_SHEETS

## 1. Purpose

This document freezes the canonical capture-sheet artifacts that back locked baselines.
A locked baseline is illegal unless it resolves to a registered capture sheet here.

## 2. Rule

Each capture sheet binds:
- one stack marker;
- one hardware floor id;
- one backend id;
- one driver family id;
- one driver posture id;
- one OS power-mode id;
- one turbo or boost posture id;
- one benchmark protocol id;
- one compiler version and target triple;
- one capture profile;
- one or more exact fixture ids captured under that sheet;
- one or more exact baseline ids captured under that sheet; and
- one or more exact capture-result ids that legally consume the same recorded posture.

The locked baseline table may reference only capture-sheet ids registered here.
No capture-sheet row in this registry may rely on wildcard, family-only, or prose-only baseline binding.

## 3. Capture Sheets

| Capture Sheet ID | Stack Marker | Hardware Floor ID | Capture Profile | Backend ID | Driver Family ID | Driver Posture ID | Power Mode ID | Turbo/Boost ID | Protocol ID | Compiler Version | Target Triple | Bound Fixture IDs | Bound Baseline IDs | Bound Capture Result IDs | Canonical notes |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| `CS-CMN-CPU-01` | `SX-CANON/1.0.24/STACK-v30` | `HF-CMN-CPU-01` | `benchmark-common-cpu` | `CPU-ONLY` | `DRV-N/A` | `DRV-N/A` | `PWR-PERF-LOCK-01` | `BOOST-OFF-01` | `BENCH-PROTO-v2` | `rustc 1.94.0` | `x86_64-pc-windows-msvc` | `lower-stack-traversal-a`, `runtime-kernel-a`, `streaming-locality-a`, `world-apply-a`, `replay-determinism-a`, `service-normalization-a`, `content-transform-a` | `BL-LS-CORE-01`, `BL-LS-ID-01`, `BL-LS-HANDLE-01`, `BL-LS-LAYOUT-01`, `BL-LS-ACCESS-01`, `BL-LS-REG-01`, `BL-LS-QUERY-01`, `BL-LS-ECS-01`, `BL-LS-SPATIAL-01`, `BL-LS-REGION-01`, `BL-STORAGE-MUT-01`, `BL-LS-MAT-01`, `BL-RT-CONT-01`, `BL-SVC-MEM-01`, `BL-MODEL-INF-01`, `BL-MODEL-GEN-01`, `BL-CONTENT-01`, `BL-REPLAY-CMP-01` | `CR-CMN-CPU-01` | CPU-only common floor |
| `CS-INT-60-CPU-01` | `SX-CANON/1.0.24/STACK-v30` | `HF-INT-60-01` | `interactive-60` | `CPU-ONLY` | `DRV-N/A` | `DRV-N/A` | `PWR-PERF-LOCK-01` | `BOOST-LOCKED-ALLCORE-01` | `BENCH-PROTO-v2` | `rustc 1.94.0` | `x86_64-pc-windows-msvc` | `runtime-kernel-a`, `streaming-locality-a`, `acoustics-streaming-a`, `startup-assembly-a` | `BL-RT-EMPTY-01`, `BL-RT-MIN-01`, `BL-RT-ORCH-01`, `BL-SVC-STREAM-INT-01`, `BL-SVC-RESID-INT-01`, `BL-AUDIO-INT-01`, `BL-STARTUP-01` | `CR-INT-60-CPU-01` | canonical interactive CPU-only capture |
| `CS-INT-60-VK-01` | `SX-CANON/1.0.24/STACK-v30` | `HF-INT-60-01` | `interactive-60` | `VK-RT-01` | `DRV-VK-STABLE-01` | `DRV-POSTURE-STABLE-01` | `PWR-PERF-LOCK-01` | `BOOST-LOCKED-ALLCORE-01` | `BENCH-PROTO-v2` | `rustc 1.94.0` | `x86_64-pc-windows-msvc` | `mixed-pressure-realtime-a`, `visibility-realtime-a`, `streaming-locality-a` | `BL-REALTIME-INT-01`, `BL-MIXED-REALTIME-INT-01`, `BL-IMG-INT-01`, `BL-SVC-XFER-INT-01` | `CR-INT-60-VK-01` | canonical interactive realtime capture |
| `CS-LH-60-CPU-01` | `SX-CANON/1.0.24/STACK-v30` | `HF-LH-60-01` | `listen-host-60` | `CPU-ONLY` | `DRV-N/A` | `DRV-N/A` | `PWR-PERF-LOCK-01` | `BOOST-LOCKED-ALLCORE-01` | `BENCH-PROTO-v2` | `rustc 1.94.0` | `x86_64-pc-windows-msvc` | `streaming-locality-a`, `network-listen-host-a`, `acoustics-streaming-a`, `destruction-heavy-a`, `agent-cycle-a` | `BL-SVC-STREAM-LH-01`, `BL-SVC-RESID-LH-01`, `BL-SVC-MEM-LH-01`, `BL-NET-TX-LH-01`, `BL-NET-SYNC-LH-01`, `BL-NET-LAT-LH-01`, `BL-KIN-01`, `BL-FIELD-01`, `BL-AGENT-01`, `BL-AUDIO-LH-01` | `CR-LH-60-CPU-01` | canonical listen-host CPU-only capture |
| `CS-LH-60-VK-01` | `SX-CANON/1.0.24/STACK-v30` | `HF-LH-60-01` | `listen-host-60` | `VK-RT-01` | `DRV-VK-STABLE-01` | `DRV-POSTURE-STABLE-01` | `PWR-PERF-LOCK-01` | `BOOST-LOCKED-ALLCORE-01` | `BENCH-PROTO-v2` | `rustc 1.94.0` | `x86_64-pc-windows-msvc` | `mixed-pressure-realtime-a`, `visibility-realtime-a`, `streaming-locality-a` | `BL-REALTIME-LH-01`, `BL-MIXED-REALTIME-LH-01`, `BL-IMG-LH-01`, `BL-SVC-XFER-LH-01` | `CR-LH-60-VK-01` | canonical listen-host realtime capture |
| `CS-HD-20-CPU-01` | `SX-CANON/1.0.24/STACK-v30` | `HF-HD-20-01` | `headless-20` | `CPU-ONLY` | `DRV-N/A` | `DRV-N/A` | `PWR-PERF-LOCK-01` | `BOOST-OFF-01` | `BENCH-PROTO-v2` | `rustc 1.94.0` | `x86_64-pc-windows-msvc` | `mixed-pressure-headless-a`, `network-headless-a`, `world-apply-a`, `streaming-locality-a` | `BL-HEADLESS-01`, `BL-MIXED-HEADLESS-01`, `BL-NET-TX-HD-01`, `BL-NET-SYNC-HD-01`, `BL-NET-LAT-HD-01`, `BL-SVC-STREAM-HD-01`, `BL-SVC-RESID-HD-01`, `BL-SVC-MEM-HD-01`, `BL-SVC-XFER-HD-01` | `CR-HD-20-CPU-01` | canonical headless capture |

## 4. Canonical Baseline-Sheet Fields

Every baseline capture sheet must record at least:
- `Capture Sheet ID`
- `Stack Marker`
- `Hardware Floor ID`
- `Capture Profile`
- `Backend ID`
- `Driver Family ID`
- `Driver Posture ID`
- `OS Power Mode ID`
- `Turbo/Boost ID`
- `Protocol ID`
- `Compiler Version`
- `Target Triple`
- exact `Fixture ID` membership
- exact `Baseline ID` membership
- exact `Capture Result ID` membership
- metric values

## 5. Legality

A performance proof is illegal when:
- a baseline row lacks a capture-sheet id;
- a capture-sheet id is missing here;
- a required fixture id, baseline id, or capture-result id is not explicitly enumerated in the registered capture-sheet row;
- the capture sheet binds a different stack marker, hardware floor, backend, driver family, or posture than the baseline row claims.