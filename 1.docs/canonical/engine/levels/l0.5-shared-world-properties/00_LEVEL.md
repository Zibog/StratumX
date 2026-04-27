# L0.5. Shared World Property Substrate

## Level Role

Cross-domain world property substrate shared by upper simulation and synthesis families.

## Crates

| Crate | Primary Role |
|---|---|
| `engine_material` | Shared world property substrate. |

## Upward Role

This level provides its canonical outputs to the next higher level without transferring away its ownership class.

## Downward Dependence

This level stays constrained by lower-layer contracts and substrates and does not bypass them.

## Threading Posture

See crate-level `31_THREADING.md` documents inside this level.

## Crate Folders

- `material/`
