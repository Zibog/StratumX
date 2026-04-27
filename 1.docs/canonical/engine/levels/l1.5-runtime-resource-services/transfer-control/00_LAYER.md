# engine_transfer_control

## Stack Position

L1.5. Runtime Resource Services

## Primary Role

transfer control.

## Canonical Scope

Runtime resource-service ownership local to this crate.

## Boundary Rationale

This crate exists so its ownership class stays explicit and does not collapse into runtime, rendering, content, or simulation families.

## Canonical Consumers

- higher simulation families, service layers, and startup where justified by local contracts.

## Downward Dependencies

See `20_DEPENDENCIES.md`.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Decode Stage | Runtime decode and unpack model. | `40_DECODE_STAGE.md` |
| Staging and Upload | Staging block and upload batch model. | `41_STAGING_AND_UPLOAD.md` |
| Fence Release Model | Fence-bound release and readback visibility model. | `42_FENCE_RELEASE_MODEL.md` |
