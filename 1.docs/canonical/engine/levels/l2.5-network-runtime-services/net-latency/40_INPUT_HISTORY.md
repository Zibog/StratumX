# Input History

## Role

Owned input history.

## Data Model

Input history is the primary rewindable client-side domain.
History length is short, bounded, compressed, and profile-visible.
Numeric values are frozen by `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Canonical History Law

- owned input history may cover at most the client-owned rewind window frozen by `constitutions/STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`;
- history entries are tick-indexed and session-bound;
- per-peer input-history bytes: <= 64 KiB;
- aggregate host history budget: <= 1 MiB on `listen-host-60` and <= 2 MiB on `headless-20`;
- history compression policy is delta-encoded, run-length compressible, and must not require whole-world context;
- whole-world or non-owned history capture is illegal.
