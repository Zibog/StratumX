# STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW

## 1. Purpose

This document freezes transport lane classes, quantization classes, domain policy defaults, numeric grids, rewind scope, host-vs-client memory split, ack ceilings, and correction ceilings.
Numeric constants are authored in `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## 2. Transport Lane Classes

- `control`: reliable ordered control/configuration lane;
- `bulk`: reliable ordered asset/session lane;
- `state`: unreliable unordered gameplay state lane;
- `input`: unreliable unordered owned-input lane.

## 3. Quantization Classes

- `exact integer`
- `bounded fixed-point`
- `bounded angular`
- `bounded normalized scalar`
- `bounded packed bitfield`
- `lossy approximate`

## 4. Domain Policy Defaults

| Export domain | Default quantization | Canonical numeric grid |
|---|---|---|
| entity, region, chunk, and session identity | exact integer | exact `u32`/`i32`/`u64` as declared by field contract |
| local position in authoritative region frame | bounded fixed-point | signed 24-bit fixed-point at 1 cm step; default export clamp <= +/-4,096 m |
| orientation and heading | bounded angular | 16-bit unsigned angle over `[0, 2π)` |
| normalized actor resources | bounded normalized scalar or exact integer | 12-bit normalized scalar or exact 8/16/32-bit integer |
| weapon or interaction flags | bounded packed bitfield | 8/16/32 bits only |
| cosmetic-only hints | lossy approximate class | explicit per-field class with minimum 10-bit mantissa equivalent |

Changing these defaults requires explicit per-field declaration.

## 5. Rewind Law

- bounded state reconstruction explicitly marked rewindable is legal;
- whole-world rewind is illegal;
- social, economy, long-range field, and content/service outputs are non-rewindable.

## 6. Ack and Delivery Ceilings

Ack windows, resend horizons, ack-state bytes, resend-buffer bytes, and delivery-state bytes are frozen by `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.
Unreliable lanes carry no reliable resend debt.

## 7. Host-vs-Client Split

On `listen-host-60`, at least 50% of per-profile network memory is reserved for host-side authoritative publication, session, and validation work, while at least 25% remains reserved for local presentation/client-facing latency work.
The remaining 25% may be shared adaptively. No remote peer may consume more than 20% of the total network memory envelope.

## 8. Correction Law

- client-owned rewind window and authoritative validation window are frozen by `STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`;
- correction bursts may not exceed the ceilings frozen by `STRATUMX_ABSOLUTE_BUDGET_CONSTITUTION.md`;
- correction of non-rewindable domains is illegal.
