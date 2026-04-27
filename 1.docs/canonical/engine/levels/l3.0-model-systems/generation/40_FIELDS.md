# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| model_descriptor | ModelDescriptor | required | model metadata semantic | owned by `engine_generation` |
| model_weights | ModelWeights | required | trained parameter binding | owned by `engine_generation` |
| generation_context | GenerationContext | required | generation request context | owned by `engine_generation` |
| generation_output | GenerationOutput | required | generated content semantic | owned by `engine_generation` |
