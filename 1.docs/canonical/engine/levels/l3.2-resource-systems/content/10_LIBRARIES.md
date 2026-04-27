# Libraries

## External Library Baseline

| Library | Role in `engine_content` |
|---|---|
| `walkdir` | File and directory traversal for content ingest. |
| `image` | Image asset decoding where required. |
| `gltf` | Structured scene/content ingest where required. |
| `serde` | Serialization for content descriptors and metadata. |
| `thiserror` | Typed content pipeline errors. |
| `smallvec` | Low-allocation pipeline metadata. |
| `tracing` | Content diagnostics. |
| `zstd` | Runtime-pack compression and seekable page product support. |

## Adjunct Compatibility

KTX2/Basis-class GPU-ready texture products are legal canonical outputs of `engine_content`, even when their encoders or transcoders live outside the minimal Rust dependency baseline.

## Fit

Each listed dependency serves the primary role of `engine_content` and stays aligned to its pressure axis.
