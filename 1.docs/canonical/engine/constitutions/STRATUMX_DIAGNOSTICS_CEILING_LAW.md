# STRATUMX_DIAGNOSTICS_CEILING_LAW

## 1. Purpose

This document freezes the legal overhead, memory, and readback limits for diagnostics.
Diagnostics are legal exceptions, not an escape hatch from performance law.

## 2. CPU Overhead Ceilings

- diagnostics overhead on `interactive-60` and `listen-host-60`: <= 3% p95 and <= 5% p99 of frame/tick wall time;
- diagnostics overhead on `headless-20`: <= 5% p95 and <= 8% p99 of tick wall time.

## 3. Memory Ceilings

- diagnostics-only shadow memory: <= 64 MiB per active profile;
- diagnostics queues must obey the same queue-depth and age ceilings as production queues unless an explicit diagnostics queue class is named in canon.

## 4. GPU and Readback Law

- low-latency presentation path readback remains illegal except in explicit diagnostics windows;
- explicit GPU readback diagnostics: <= 1 readback batch every 30 realtime frames and <= 8 MiB per readback batch;
- diagnostics may not force pipeline-cache invalidation or residency-class promotion outside declared diagnostics windows.

## 5. Rule

A diagnostics mode that breaches the ceilings above is not canonical diagnostics. It is a separate profiling mode and must not be used to certify gold-profile legality.
