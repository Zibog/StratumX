# engine_inference

## Stack Position

L3.0. Model Systems

## Primary Role

Model inference boundary.

## Canonical Scope

Context assembly, model adapters, structured outputs, and inference budget model.

## Boundary Rationale

Inference is a model-facing service class and should stay above critical simulation while remaining tightly integrated through explicit contracts.

## Canonical Consumers

- `engine_runtime`
- `engine_generation`
- `engine_startup`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_world` — World truth boundary.
- `engine_ecs` — ECS substrate.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Context Assembly | Structured assembly of model-facing context. | `40_CONTEXT_ASSEMBLY.md` |
| Model Adapters | Backend adapter surface for inference. | `41_MODEL_ADAPTERS.md` |
| Structured Outputs | Normalized output model returned from inference. | `42_STRUCTURED_OUTPUTS.md` |
| Budget Model | Budget and cadence model for inference work. | `43_BUDGET_MODEL.md` |


## Boundary Preservation

See `32_BOUNDARY_PRESERVATION.md` for the canonical relation between this crate and earlier canon boundaries.
