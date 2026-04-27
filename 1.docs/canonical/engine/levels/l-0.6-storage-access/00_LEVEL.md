# L-0.6. Storage Access

## Level Role

Read/write access boundaries over storage layout.

## Crates

| Crate | Primary Role |
|---|---|
| `engine_storage_access` | Deterministic read/write access model over storage layout. |

## Upward Role

This level provides its canonical outputs to the next higher level without transferring away its ownership class.

## Downward Dependence

This level stays constrained by lower-layer contracts and substrates and does not bypass them.

## Threading Posture

See crate-level `31_THREADING.md` documents inside this level.

## Crate Folders

- `storage-access/`
