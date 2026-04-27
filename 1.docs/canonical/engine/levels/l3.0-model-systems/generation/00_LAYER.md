# engine_generation

## Stack Position

L3.0. Model Systems

## Primary Role

Model-driven generation boundary.

## Canonical Scope

Generation context expansion, generation products, transformation pipelines, and output packaging.

## Boundary Rationale

Generation is a model-facing synthesis class and should stay above critical simulation while remaining general-purpose and engine-facing.

## Canonical Consumers

- `engine_runtime`
- `engine_startup`

## Downward Dependencies

- `engine_core` — Base contracts.
- `engine_world` — World truth boundary.
- `engine_inference` — Inference boundary for model-backed generation.

## Threading Posture

See `31_THREADING.md`.

## Local Glossary

| Element | Role | Canonical Document |
|---|---|---|
| Context Expansion | Expansion of context into generation-ready form. | `40_CONTEXT_EXPANSION.md` |
| Generation Products | Canonical outputs of generation work. | `41_GENERATION_PRODUCTS.md` |
| Transformation Pipelines | Post-generation transformation stages. | `42_TRANSFORMATION_PIPELINES.md` |
| Output Packaging | Packaging of generated outputs for consumers. | `43_OUTPUT_PACKAGING.md` |


## Boundary Preservation

See `32_BOUNDARY_PRESERVATION.md` for the canonical relation between this crate and earlier canon boundaries.
