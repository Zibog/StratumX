# STRATUMX_STREAMING_AND_PACK_LOCALITY_LAW

## 1. Purpose

This document freezes pack locality, page law, cancellation law, read amplification ceilings, verification ceilings, retry law, decompression throughput, and the runtime compression/transcode matrix.

## 2. Canonical Pack Law

- runtime packs are immutable, versioned, and chunk-addressable;
- pack page target size is 128 KiB compressed with a hard cap of 256 KiB compressed for canonical runtime pages;
- page lookup must be deterministic and locality-aware;
- page identity must be stable across startup-mediated binding and deterministic replay.

## 3. Runtime Compression / Transcode Matrix

| Product class | Canonical runtime rule |
|---|---|
| GPU textures | GPU-ready or transcode-ready supercompressed pages only; raw source decode at runtime is illegal |
| geometry buffers | startup-ready or background-prepared binary pages only; per-frame raw mesh import is illegal |
| material tables and shader-derived metadata | startup-only or background-cached preparation; low-latency path runtime compilation is illegal |
| physics, collision, navigation, and destruction substrate | cooked binary products only; runtime cook is illegal |
| streaming audio pages | seekable compressed pages are legal, but decode is acoustics-owned only and illegal on runtime/apply lanes |

## 4. Locality Law

- stream requests are grouped first by region/chunk locality, then by resource class, then by urgency;
- one runtime page may not span more than 2 adjacent chunk neighborhoods in its primary locality group;
- prefetch is legal only with explicit locality score and budget visibility;
- seek locality metric must be observable in diagnostics.

## 5. Verification and Retry Law

- page verification CPU cost must obey `STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`;
- corruption retry budget: <= 2 retries per page before quarantine;
- retry storm law: repeated verification failures must back off exponentially and may not consume more than 5% of the stream budget for 30 seconds;
- manifest or page quarantine must be visible to startup/runtime diagnostics.

## 6. Cancellation Law

- canceled requests must leave ordered visibility records;
- unused prefetch work must be cancelable before decode, before staging, and before upload;
- cancellation debt may not accumulate silently across frames.

## 7. Read Amplification and Decompression Law

- steady-state read amplification ceiling is governed by `STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`;
- decompression throughput on benchmark-floor hardware must remain >= 600 MiB/s per core for canonical page products;
- pack layout that violates these ceilings is illegal.

## 8. Optional Backend Law

Seekable multi-frame zstd products are canonical-compatible.
Backend-specific IO engines, including Linux `io_uring`, are optional acceleration paths only and may not change ownership or completion-visibility law.
