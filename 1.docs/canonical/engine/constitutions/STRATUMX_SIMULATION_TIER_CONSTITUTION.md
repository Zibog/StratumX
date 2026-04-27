# STRATUMX_SIMULATION_TIER_CONSTITUTION

## 1. Purpose

This document freezes simulation tiers, cadence classes, promotion/demotion legality, and near-footprint ceilings.

## 2. Tier Classes

- `N`: near tier, full authoritative cadence and rich local interaction;
- `M`: mid tier, reduced cadence and reduced fidelity;
- `F`: far tier, statistical, event-driven, or demand-driven simulation only.

## 3. Cadence Law

Near, mid, and far cadence classes must be explicit and observable. Whole-world near-tier execution is illegal.

## 4. Near-Footprint Ceilings

| Profile | Near regions | Near actors | Near destructible cells |
|---|---:|---:|---:|
| `interactive-60` | <= 9 active near regions | <= 384 near actors | <= 65,536 near destructible cells |
| `listen-host-60` | <= 24 merged near regions across all local+remote authorities | <= 1,024 near actors | <= 131,072 near destructible cells |
| `headless-20` | <= 16 active near regions unless explicit server benchmark profile says otherwise | <= 1,024 near actors | <= 131,072 near destructible cells |

## 5. Fairness Law for `listen-host-60`

- at least 50% of the total near-region and near-actor footprint is reserved for the local presentation authority;
- no single remote authority may consume more than 25% of the total near-region or near-actor footprint;
- remote authorities beyond the legal share must demote first to `M` or `F` before the local presentation authority is demoted.

## 6. Hysteresis Law

Promotion and demotion must be explicit, delayed, and budget-visible. One-frame or one-tick tier thrash is illegal.
