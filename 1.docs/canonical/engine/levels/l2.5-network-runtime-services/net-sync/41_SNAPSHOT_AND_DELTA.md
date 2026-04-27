# Snapshot and Delta

## Role

Authoritative sync product model.

## Data Model

Dirty sets, snapshot sets, and delta sets over authoritative exported state.
All exported fields declare quantization class and export class. Per-connection bandwidth remains budget-bounded and dirty-set coalescing is mandatory.
Numeric grids are frozen by `constitutions/STRATUMX_NETWORK_QUANTIZATION_AND_REWIND_LAW.md` and `constitutions/STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md`.

## Domain Policy Defaults

| Export domain | Default quantization | Canonical grid |
|---|---|---|
| entity, region, chunk, and session identity | exact integer | exact `u32`/`i32`/`u64` |
| local position in authoritative region frame | bounded fixed-point | signed 24-bit fixed-point, 1 cm step, default export clamp <= +/-4,096 m |
| orientation and heading | bounded angular | 16-bit unsigned angle over `[0, 2π)` |
| normalized actor resources | bounded normalized scalar or exact integer | 12-bit normalized scalar or exact 8/16/32-bit integer |
| weapon or interaction flags | bounded packed bitfield | 8/16/32 bits only |
| cosmetic-only hints | lossy approximate class | explicit per-field class, minimum 10-bit mantissa equivalent |

Changing these defaults requires explicit per-field declaration.
