# L4. Startup

## Level Role

Assembly, profile selection, and runtime launch.

## Crates

| Crate | Primary Role |
|---|---|
| `engine_startup` | Startup and assembly. |

## Upward Role

This level provides its canonical outputs to the next higher level without transferring away its ownership class.

## Downward Dependence

This level stays constrained by lower-layer contracts and substrates and does not bypass them.

## Threading Posture

See crate-level `31_THREADING.md` documents inside this level.

## Crate Folders

- `startup/`
