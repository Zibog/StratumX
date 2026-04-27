# Residency Sets

## Role

Runtime residency classes.

## Data Model

Canonical residency sets are `hot`, `warm`, `cold`, and `quarantined`.
Promotion and demotion use explicit hysteresis and never oscillate purely on one-frame demand noise.
All numeric windows are frozen by `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Canonical Residency Law

- `hot` owns assets that are legally bound in a current critical execution lane, current presentation window, or current authoritative publication window;
- `warm` owns assets with explicit locality evidence for the next legal window;
- `cold` owns assets that are evictable without breaking correctness;
- `quarantined` owns assets waiting for fence or completion visibility before publication or release.

## Transition Rule

- `cold -> warm` and `warm -> hot` use only locality evidence or startup-selected binds;
- `hot -> warm` and `warm -> cold` use only the hysteresis windows frozen in the numeric source of truth;
- `quarantined -> legal publish` is illegal before completion visibility and the canonical grace window;
- local documents may not redefine the windows numerically.

## Illegal Patterns

- one-frame oscillation between `hot` and `warm`;
- promotion without locality evidence;
- release from `quarantined` before fence-visible completion.
