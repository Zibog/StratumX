# STRATUMX_NUMERIC_VALIDATION_RULE

## Purpose

This document freezes the package-contained numeric-validation contract.

## Rule

Shared hard numbers are legal only when all of the following exist together:
- `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`;
- the validator artifact contract;
- the package-contained validator result artifact for the current stack marker.

## Validator Contract

The validator must:
- scan the registered hardened affected set and constitutional numeric set;
- resolve every shared hard number to a cited numeric-source row;
- reject uncited repeated shared constants;
- reject conflicting repeated shared constants;
- emit a deterministic result artifact bound to the current stack marker.

## Result Contract

The result artifact must record:
- validator run id;
- stack marker;
- deterministic scanned file manifest and file count;
- cited numeric-authority row count;
- local-only numeric declaration count;
- drift errors;
- uncited shared-number errors;
- missing-source-row errors;
- final validator status;
- deterministic artifact digest or equivalent run hash.

## Legality

Numeric governance is package-proven only when both the validator artifact and the validator result artifact are present and agree on stack marker and scanned scope.
