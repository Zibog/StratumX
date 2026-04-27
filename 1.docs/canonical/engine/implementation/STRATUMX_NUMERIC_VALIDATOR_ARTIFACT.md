# STRATUMX_NUMERIC_VALIDATOR_ARTIFACT

## Purpose

This document freezes the package-contained validator artifact required by `STRATUMX_NUMERIC_VALIDATION_RULE.md`.

## Artifact Contract

The implementation handoff must include a validator artifact named `stratumx_numeric_validator` that performs at minimum:
- numeric-authority citation scan;
- repeated-constant extraction;
- shared-versus-local-only classification;
- drift detection against `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`;
- rejection of uncited shared hard numbers.

## Input Surface

The validator must read:
- `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`
- all registered hardened affected local docs listed by `ROOT-011`
- any constitutional document that introduces or repeats shared hard numbers

## Output Contract

The validator must emit a deterministic report containing:
- deterministic scanned file manifest in canonical path order and deterministic file count
- cited numeric-authority rows
- local-only numeric declarations
- drift errors
- uncited shared-number errors
- missing-source-row errors
- final pass/fail status
- artifact digest or deterministic run hash

## Handoff Rule

Implementation handoff is incomplete when this validator artifact contract is absent, when the package-contained validator result is absent, or when package-local numeric governance cannot be validated against them.
