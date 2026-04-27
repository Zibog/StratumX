# L3.1. Synthesis Systems

## Level Role

Perception-facing synthesis systems for image and acoustic outputs.

## Crates

| Crate | Primary Role |
|---|---|
| `engine_imaging` | Image synthesis family. |
| `engine_acoustics` | Acoustic synthesis family. |

## Upward Role

This level provides its canonical outputs to the next higher level without transferring away its ownership class.

## Downward Dependence

This level stays constrained by lower-layer contracts and substrates and does not bypass them.

## Threading Posture

See crate-level `31_THREADING.md` documents inside this level.

## Crate Folders

- `imaging/`
- `acoustics/`
