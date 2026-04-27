# STRATUMX_TEST_SEVERITY_SYSTEM

## 1. Purpose

This document defines the canonical severity and result system for StratumX tests.

## 2. Failure Classes

- `FAIL_BOUNDARY` — public contract or boundary corruption.
- `FAIL_STATE` — authoritative state, snapshot, apply, or restoration corruption.
- `FAIL_DETERMINISM` — deterministic progression law violation.
- `FAIL_CONCURRENCY` — illegal threading or window law violation.
- `FAIL_PERFORMANCE` — hard performance law violation.

## 3. Block Classes

- `B5_CRITICAL_STOP` — stop execution and fail immediately.
- `B4_MERGE_BLOCK` — code may not merge.
- `B3_RELEASE_BLOCK` — release candidate is illegal.
- `B2_PROFILE_REJECT` — profile or assembly is illegal.
- `B1_REGRESSION_ALERT` — regression is recorded and must be triaged.

## 4. Warning Classes

- `W5_STRUCTURAL`
- `W4_PERF_RISK`
- `W3_STABILITY_RISK`
- `W2_COVERAGE_GAP`
- `W1_OBSERVATION_NOTE`

## 5. Pass Strength Classes

- `P5_CANON_LOCK`
- `P4_STRONG_PROOF`
- `P3_BASELINE_PASS`
- `P2_SMOKE_PASS`
- `P1_OBSERVED_PASS`

## 6. Tier Model

Every canonical test category carries:
- Tier 1 hard gate;
- Tier 2 stretch target.

Tier 1 is merge/release relevant.
Tier 2 is a stronger target that may exceed current implementation but remains canonical direction.

## 7. Default Severity Bindings by Core Class

| Core Test Class | Default Failure Class | Default Block Class | Default Warning Class | Default Pass Strength |
|---|---|---|---|---|
| structural | FAIL_BOUNDARY | B4_MERGE_BLOCK | W5_STRUCTURAL | P3_BASELINE_PASS |
| contract | FAIL_BOUNDARY | B4_MERGE_BLOCK | W5_STRUCTURAL | P4_STRONG_PROOF |
| invariants | FAIL_STATE | B5_CRITICAL_STOP | W3_STABILITY_RISK | P5_CANON_LOCK |
| determinism | FAIL_DETERMINISM | B5_CRITICAL_STOP | W3_STABILITY_RISK | P5_CANON_LOCK |
| concurrency legality | FAIL_CONCURRENCY | B5_CRITICAL_STOP | W3_STABILITY_RISK | P4_STRONG_PROOF |
| property | FAIL_STATE | B4_MERGE_BLOCK | W2_COVERAGE_GAP | P4_STRONG_PROOF |
| fault and degradation | FAIL_BOUNDARY or FAIL_STATE | B3_RELEASE_BLOCK | W3_STABILITY_RISK | P3_BASELINE_PASS |
| performance smoke | FAIL_PERFORMANCE | B4_MERGE_BLOCK or B3_RELEASE_BLOCK | W4_PERF_RISK | P2_SMOKE_PASS |

## 8. Default Severity Bindings by Operational Subclass

| Operational Subclass | Canonical Parent Class | Default Failure Class | Default Block Class | Default Warning Class | Default Pass Strength |
|---|---|---|---|---|---|
| degradation | fault and degradation | FAIL_STATE | B3_RELEASE_BLOCK | W3_STABILITY_RISK | P3_BASELINE_PASS |
| enablement legality | contract | FAIL_BOUNDARY | B2_PROFILE_REJECT | W5_STRUCTURAL | P4_STRONG_PROOF |
| restoration legality | invariants | FAIL_STATE | B3_RELEASE_BLOCK | W3_STABILITY_RISK | P4_STRONG_PROOF |
| compatibility rejection | contract | FAIL_BOUNDARY | B3_RELEASE_BLOCK | W3_STABILITY_RISK | P4_STRONG_PROOF |
| benchmark gate | performance smoke | FAIL_PERFORMANCE | B4_MERGE_BLOCK or B3_RELEASE_BLOCK | W4_PERF_RISK | P4_STRONG_PROOF |

## 9. Canonical Rule

Every canonical test and benchmark record must carry:
- core canonical test class;
- operational subclass when applicable;
- failure class when failed;
- block class when blocking;
- warning class when warning-only;
- pass strength when passed;
- Tier 1 and Tier 2 result state.
