# STRATUMX_REPLAY_AND_DETERMINISM_CONSTITUTION

## 1. Purpose

This document freezes deterministic replay scope, checkpoint cadence, checkpoint size, save/join handoff size, divergence law, and service-layer replay constraints.
Numeric constants are authored in `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## 2. Canonical Replay Scope

Deterministic mode covers:
- authoritative world snapshots;
- staged outputs and apply-visible mutations;
- runtime-owned progression order;
- network publication inputs that are declared replay-relevant;
- authoritative checkpoints at legal intervals.

## 3. Canonical Checkpoint Cadence

| Item | Cadence |
|---|---|
| full authoritative checkpoint on `interactive-60` / `listen-host-60` | every 100 authoritative ticks |
| full authoritative checkpoint on `headless-20` | every 50 authoritative ticks |
| checkpoint compare cadence on `interactive-60` / `listen-host-60` | every 20 authoritative ticks |
| checkpoint compare cadence on `headless-20` | every 10 authoritative ticks |
| mandatory full checkpoint | startup bind, save boundary, join handoff, diagnostics boundary |

## 4. Payload Ceilings

- full checkpoint size ceiling, replay log bytes per tick ceiling, save-boundary package ceiling, and join-handoff replay payload ceiling are frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`;
- same-tick bridge payloads are logged at the bridge record granularity, not collapsed into one opaque blob;
- full checkpoints must carry enough state to resume deterministic playback without hidden side channels.

## 5. Service-Layer Law

- `engine_inference` and `engine_generation` may not write world truth directly;
- their outputs must be typed, staged, and replay-stable or explicitly excluded from deterministic mode;
- replay-excluded service outputs may not silently leak into authoritative world mutation.

## 6. Divergence Law

- divergence detection must occur at legal checkpoints;
- divergence reports must identify phase, crate, checkpoint id, and fixture/profile id;
- acceptance of silent divergence is illegal.
