# L-0.05. World Region

## Level Role

Region and chunk substrate for locality, partitioning, and dirty tracking.

## Crates

| Crate | Primary Role |
|---|---|
| `engine_world_region` | Region and chunk substrate. |

## Upward Role

This level provides its canonical outputs to the next higher level without transferring away its ownership class.

## Downward Dependence

This level stays constrained by lower-layer contracts and substrates and does not bypass them.

## Threading Posture

See crate-level `31_THREADING.md` documents inside this level.

## Crate Folders

- `world-region/`
