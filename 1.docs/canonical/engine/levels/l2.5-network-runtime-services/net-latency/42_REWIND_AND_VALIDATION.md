# Rewind and Validation

## Role

Short-history authoritative validation.

## Data Model

Numeric Authority: STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md §9, §10


Rewind is legal only for declared rewindable domains and within bounded windows.
Whole-world rewind, long-range social rewind, and service-layer rewind are illegal.
Numeric windows are frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` and `STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md`.

## Canonical Windows

| Window | Ceiling |
|---|---:|
| client-owned rewind | <= 250 ms |
| authoritative hit-validation rewind | <= 500 ms |

## Canonical Rule

Rewind beyond the ceilings above or outside explicitly rewindable domains is illegal.
