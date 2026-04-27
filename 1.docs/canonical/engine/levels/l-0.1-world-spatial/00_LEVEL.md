# L-0.1. World Spatial

## Level Role

Spatial substrate for coordinates, transforms, and world-space relations.

## Crates

| Crate | Primary Role |
|---|---|
| `engine_world_spatial` | World-specific spatial substrate. |

## Upward Role

This level provides its canonical outputs to the next higher level without transferring away its ownership class.

## Downward Dependence

This level stays constrained by lower-layer contracts and substrates and does not bypass them.

## Threading Posture

See crate-level `31_THREADING.md` documents inside this level.

## Crate Folders

- `world-spatial/`
