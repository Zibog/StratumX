# L-0.2. ECS Assembly

## Level Role

Assembly surface that binds lower ECS substrate into one ECS-facing boundary.

## Crates

| Crate | Primary Role |
|---|---|
| `engine_ecs` | Assembled ECS substrate. |

## Upward Role

This level provides its canonical outputs to the next higher level without transferring away its ownership class.

## Downward Dependence

This level stays constrained by lower-layer contracts and substrates and does not bypass them.

## Threading Posture

See crate-level `31_THREADING.md` documents inside this level.

## Crate Folders

- `ecs/`
