# Dream Stack Asset Family Canonicalization And Import Validation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Make heavy-domain import law exact per asset family.

## Asset families
| Family | Validation examples | Canonical output | Owning consumers |
|---|---|---|---|
| terrain heightfield | bounds, scale, hole legality | `asset.terrain.heightfield.canon` | editor `119`, engine `111`, tooling `83–84` |
| terrain texture set | layer count, tiling law, channel packing | `asset.terrain.texture_set.canon` | editor `119`, `122`, engine `130` |
| destruction mask / cross-section atlas | class ids, topology rows, atlas completeness | `asset.destruction.mask_pack.canon` | editor `120`, `122`, engine `110`, `134` |
| fur coverage map | coverage channels, wet/char compatibility | `asset.fur.coverage_pack.canon` | editor `125`, engine `127` |
| audio bank | sample rate class, loop/variation law | `asset.audio.bank.canon` | editor `135`, engine `128` |
| weather preset field | field schema and timeline curve validity | `asset.weather.field_pack.canon` | editor `136`, engine `114` |
| motion prior / contact constraints | skeleton family validity, body-class tags | `asset.motion.prior_pack.canon` | editor `134`, engine `123` |

## Failure taxonomy
`IMP_*` source errors, `AST_*` family errors, `NRM_*` normalization errors, `CMP_*` compatibility errors.
