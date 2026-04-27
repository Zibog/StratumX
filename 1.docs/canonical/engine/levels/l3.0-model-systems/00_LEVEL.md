# L3.0. Model Systems

## Level Role

Model-facing compute systems for inference and generation.

## Crates

| Crate | Primary Role |
|---|---|
| `engine_inference` | Model inference boundary. |
| `engine_generation` | Model-driven generation boundary. |

## Upward Role

This level provides its canonical outputs to the next higher level without transferring away its ownership class.

## Downward Dependence

This level stays constrained by lower-layer contracts and substrates and does not bypass them.

## Threading Posture

See crate-level `31_THREADING.md` documents inside this level.

## Crate Folders

- `inference/`
- `generation/`
