# Refcount and Budgets

## Role

Residency accounting and budget coordination.

## Data Model

Residency accounting combines refcount, pressure, and age. No single scalar may unilaterally decide promotion or eviction.
Per-profile residency obeys the RAM and VRAM ceilings frozen by `constitutions/STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md` and the class ceilings frozen by `constitutions/STRATUMX_MEMORY_AND_RESIDENCY_CONSTITUTION.md`.
Numeric windows and batch ceilings are frozen by `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Canonical Rule Matrix

| Signal | Legal use |
|---|---|
| refcount | pin active ownership and prevent illegal eviction |
| age | demotion and eviction ordering inside the same residency class |
| pressure | trigger budget ladder and deny optional promotions |
| locality score | choose among legal warm candidates |

## Additional Ceilings

- eviction batch size may not exceed the batch ceiling frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`;
- promotion debt may not exceed the promotion-debt ceiling frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Illegal Patterns

- refcount-only hot retention;
- one-frame warm->hot->warm oscillation;
- hidden VRAM shadow growth outside residency accounting.
