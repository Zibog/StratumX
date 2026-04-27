# Communication

## Inputs

Structured context, world/ECS-derived inputs, and runtime-issued inference work windows.

## Outputs

Structured model results, normalized outputs, and inference diagnostics.

## Canonical Data Forms

- context payloads
- model requests
- structured outputs
- diagnostics

## Upward Flow

`engine_inference` communicates upward through the listed canonical data forms rather than by hidden ownership transfer.
