# STRATUMX_ENGINE_BUDGET_LEDGER

## 1. Purpose

This document freezes the end-to-end engine-owned budget ledger.
It binds every canonical budget class to its gate rows, legal baseline bindings or absolute gates, and measured headroom posture per profile.

## 2. Ledger Rule

A profile is legal only when:
- every class budget remains inside its class ceiling;
- every cited regression gate resolves to a same-profile locked baseline row or to an explicitly legal `benchmark-common-cpu` row;
- every cited absolute gate resolves to `ABSOLUTE-GATE` and stays inside the frozen hard ceiling;
- the integrated mixed-pressure envelope remains legal for the same profile;
- the end-to-end headroom table remains non-negative for the same profile.

## 3. Interactive-60 Numeric Ledger

| Budget class | Class ceiling p95 | Bound gate rows | Bound baseline rows / absolute gates | Reserved headroom after class ceiling binding |
|---|---:|---|---|---:|
| runtime kernel + realtime shell | 0.85 ms | orchestration; realtime whole-profile; low-latency queue | `BL-RT-ORCH-01`; `BL-REALTIME-INT-01`; `ABSOLUTE-GATE` | 0.18 ms |
| `L1.5` aggregate | 1.60 ms | stream; residency; memory; transfer; queue stability | `BL-SVC-STREAM-INT-01`; `BL-SVC-RESID-INT-01`; `BL-SVC-MEM-01`; `BL-SVC-XFER-INT-01`; `ABSOLUTE-GATE` | 0.25 ms |
| imaging | 2.25 ms | imaging extraction; integrated mixed realtime envelope | `BL-IMG-INT-01`; `BL-MIXED-REALTIME-INT-01` | 0.40 ms |
| acoustics | 0.90 ms | acoustics solve; integrated mixed realtime envelope | `BL-AUDIO-INT-01`; `BL-MIXED-REALTIME-INT-01` | 0.15 ms |
| world snapshot/apply | 2.10 ms | snapshot build; apply batch integration | `ABSOLUTE-GATE` | 0.30 ms |
| replay + diagnostics reserve | 0.85 ms | replay cadence; replay compare; diagnostics probe; degradation probe | `ABSOLUTE-GATE`; `BL-REPLAY-CMP-01` | 0.10 ms |
| mandatory simulation families aggregate | 6.80 ms | integrated mixed realtime envelope | `BL-MIXED-REALTIME-INT-01` | 1.00 ms |
| **engine-owned aggregate** | **16.00 ms** | integrated mixed realtime envelope | `BL-MIXED-REALTIME-INT-01` | **2.50 ms residual to engine-owned ceiling at 13.50 ms measured p95** |

## 4. Listen-Host-60 Numeric Ledger

| Budget class | Class ceiling p95 | Bound gate rows | Bound baseline rows / absolute gates | Reserved headroom after class ceiling binding |
|---|---:|---|---|---:|
| runtime kernel + realtime shell | 0.95 ms | realtime whole-profile; low-latency queue | `BL-REALTIME-LH-01`; `ABSOLUTE-GATE` | 0.15 ms |
| `L1.5` aggregate | 1.80 ms | stream; residency; memory; transfer; queue stability | `BL-SVC-STREAM-LH-01`; `BL-SVC-RESID-LH-01`; `BL-SVC-MEM-LH-01`; `BL-SVC-XFER-LH-01`; `ABSOLUTE-GATE` | 0.25 ms |
| `L2.5` aggregate | 1.40 ms | transport; sync publication; ack backlog; latency | `BL-NET-TX-LH-01`; `BL-NET-SYNC-LH-01`; `ABSOLUTE-GATE`; `BL-NET-LAT-LH-01` | 0.20 ms |
| mandatory simulation families | 7.20 ms | kinetics; field; agents; tier envelope | `BL-KIN-01`; `BL-FIELD-01`; `BL-AGENT-01`; `ABSOLUTE-GATE` | 0.90 ms |
| imaging | 2.25 ms | imaging extraction; integrated mixed realtime envelope | `BL-IMG-LH-01`; `BL-MIXED-REALTIME-LH-01` | 0.35 ms |
| acoustics | 0.90 ms | acoustics solve; integrated mixed realtime envelope | `BL-AUDIO-LH-01`; `BL-MIXED-REALTIME-LH-01` | 0.12 ms |
| world + replay + diagnostics reserve | 3.35 ms | world absolute; replay cadence; replay compare; diagnostics probe; degradation probe | `ABSOLUTE-GATE`; `BL-REPLAY-CMP-01` | 0.28 ms |
| **engine-owned aggregate** | **17.85 ms** | integrated mixed realtime envelope | `BL-MIXED-REALTIME-LH-01` | **4.35 ms residual to engine-owned ceiling at 13.50 ms measured p95** |

## 5. Headless-20 Numeric Ledger

| Budget class | Class ceiling p95 | Bound gate rows | Bound baseline rows / absolute gates | Reserved headroom after class ceiling binding |
|---|---:|---|---|---:|
| runtime kernel + headless shell | 1.10 ms | headless whole-profile; integrated mixed headless envelope | `BL-HEADLESS-01`; `BL-MIXED-HEADLESS-01` | 0.18 ms |
| `L1.5` aggregate | 2.20 ms | stream; residency; memory; transfer; queue stability | `BL-SVC-STREAM-HD-01`; `BL-SVC-RESID-HD-01`; `BL-SVC-MEM-HD-01`; `BL-SVC-XFER-HD-01`; `ABSOLUTE-GATE` | 0.30 ms |
| `L2.5` aggregate | 1.70 ms | transport; sync publication; ack backlog; latency | `BL-NET-TX-HD-01`; `BL-NET-SYNC-HD-01`; `ABSOLUTE-GATE`; `BL-NET-LAT-HD-01` | 0.25 ms |
| mandatory simulation families | 12.00 ms | integrated mixed headless envelope; tier envelope | `BL-MIXED-HEADLESS-01`; `ABSOLUTE-GATE` | 1.20 ms |
| world + replay + storage reserve | 4.00 ms | world absolute; replay cadence; replay compare; storage ceilings | `ABSOLUTE-GATE`; `BL-REPLAY-CMP-01`; `BL-MIXED-HEADLESS-01` | 0.35 ms |
| **engine-owned aggregate** | **22.50 ms** | integrated mixed headless envelope | `BL-MIXED-HEADLESS-01` | **5.00 ms residual to engine-owned ceiling at 17.50 ms measured p95** |

## 6. Machine-Style End-to-End Headroom Table

| Profile | Sum of class ceilings | Unallocated profile reserve | Engine-owned ceiling | Measured integrated p95 | Residual to summed class ceilings | Residual to engine-owned ceiling | Residual to legal envelope |
|---|---:|---:|---:|---:|---:|---:|---:|
| `interactive-60` | 15.35 ms | 0.65 ms | 16.00 ms | 13.50 ms | 1.85 ms | 2.50 ms | 0.00 ms to `BL-MIXED-REALTIME-INT-01` envelope |
| `listen-host-60` | 17.85 ms | 0.00 ms | 17.85 ms | 13.50 ms | 4.35 ms | 4.35 ms | 0.00 ms to `BL-MIXED-REALTIME-LH-01` envelope |
| `headless-20` | 21.00 ms | 1.50 ms | 22.50 ms | 17.50 ms | 3.50 ms | 5.00 ms | 22.50 ms to `BL-MIXED-HEADLESS-01` envelope |

## 7. Binding Closure Rule

No class may consume reserve that belongs to another class.
A profile is illegal when any row above resolves to a baseline row owned by another profile floor, when any absolute gate substitutes for a missing regression baseline, or when any residual headroom becomes negative.
