# STRATUMX_TESTING_CONSTITUTION

## 1. Purpose

This document defines the canonical testing law of StratumX.

## 2. Testing Principles

- every layer has mandatory tests;
- deterministic behavior is a first-class test target;
- critical simulation families are tested as world-progression participants;
- service layers are tested as legal optional or degradable systems;
- performance regressions are testable events;
- reporting must preserve failure class, block class, warning class, and pass strength.

## 3. Canonical Test Taxonomy

### 3.1. Core canonical test classes
- structural
- contract
- invariants
- determinism
- concurrency legality
- property
- fault and degradation
- performance smoke

### 3.2. Canonical operational subclasses
Operational subclasses refine core classes and remain canonically bound to them.

| Operational Subclass | Canonical Parent Class | Canonical Purpose |
|---|---|---|
| degradation | fault and degradation | degraded optional-unit behavior remains legal and visible |
| enablement legality | contract | legal enablement sets and illegal disablement rejection |
| restoration legality | invariants | legal restoration decision, restoration scope, and restoration state correctness |
| compatibility rejection | contract | illegal compatibility combinations are rejected explicitly |
| benchmark gate | performance smoke | measured performance hard-gate and stretch-target validation |
| microbench regression | performance smoke | statistical regression proof for critical execution lanes |
| concurrency permutation | concurrency legality | model-checked runtime primitive legality under legal interleavings |
| replay proof | determinism | deterministic replay, checkpoint, and divergence validation |

## 4. Mandatory Coverage by Layer

Use:
- `STRATUMX_LAYER_TEST_MATRIX.md`
- `STRATUMX_CRATE_TEST_MATRIX.md`
- `STRATUMX_TEST_SEVERITY_SYSTEM.md`

## 5. Zero-Tolerance Expectations

- zero silent authority drift;
- zero illegal family or mandatory-unit disablement;
- zero silent nondeterministic progression in deterministic mode;
- zero silent degradation affecting legal execution;
- zero silent compatibility acceptance when compatibility law rejects;
- zero silent restoration outside legal restoration scope.

## 6. Canonical Rule

Every canonical test must report:
- core canonical test class;
- operational subclass when applicable;
- failure class when failed;
- block class when blocking;
- warning class when warning-only;
- pass strength when passed;
- Tier 1 hard gate and Tier 2 stretch target where applicable.
