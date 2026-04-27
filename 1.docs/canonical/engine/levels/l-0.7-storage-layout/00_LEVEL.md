# L-0.7. Storage Layout

## Level Role

Physical organization of ECS-oriented storage.

## Crates

| Crate | Primary Role |
|---|---|
| `engine_storage_layout` | Physical organization of storage. |

## Upward Role

This level provides its canonical outputs to the next higher level without transferring away its ownership class.

## Downward Dependence

This level stays constrained by lower-layer contracts and substrates and does not bypass them.

## Threading Posture

See crate-level `31_THREADING.md` documents inside this level.

## Crate Folders

- `storage-layout/`
