# STRATUMX_LAYER_TEST_MATRIX

## 1. Purpose

This document defines mandatory test classes by canonical layer.

## 2. Matrix

| Layer | Mandatory Core Test Classes | Mandatory Operational Subclasses | Hard Invariants |
|---|---|---|---|
| lower substrate | structural, contract, invariants, determinism, performance smoke | benchmark gate | fixed substrate legality, no hidden world authority |
| L0.5 shared world property substrate | structural, contract, invariants, concurrency legality, performance smoke | benchmark gate | immutable shared property truth |
| L1 runtime family | structural, contract, invariants, determinism, concurrency legality, fault and degradation, performance smoke | degradation, benchmark gate | one active runtime authority, phase legality, apply legality |
| L1.5 runtime resource services | structural, contract, invariants, concurrency legality, fault and degradation, performance smoke | pressure, residency, queue, benchmark gate | no hidden authority transfer, no illegal allocator or residency ownership leakage |
| L2 critical simulation families | structural, contract, invariants, determinism, concurrency legality, property, performance smoke | benchmark gate | legal staged deltas only |
| L2.5 network runtime services | structural, contract, invariants, determinism, concurrency legality, fault and degradation, performance smoke | role legality, compatibility rejection, benchmark gate | server authority preserved, ordered publication where required |
| L3 service layers | structural, contract, invariants, fault and degradation, performance smoke | enablement legality, degradation, compatibility rejection, benchmark gate | optional-layer legality, isolation from critical progression law, explicit degradation visibility |
| L4 startup | structural, contract, invariants, performance smoke | restoration legality, compatibility rejection, enablement legality | legal assembly decision, legal enablement set, legal restoration decision |

## 3. Canonical Rule

No canonical layer may ship without the mandatory core classes and required operational subclasses listed here.
