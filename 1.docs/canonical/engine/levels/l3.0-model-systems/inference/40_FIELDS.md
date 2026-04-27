# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| inference_request | InferenceRequest | required | inference input semantic | owned by `engine_inference` |
| inference_model | InferenceModel | required | loaded model reference | owned by `engine_inference` |
| inference_result | InferenceResult | required | inference output semantic | owned by `engine_inference` |
| inference_batch | InferenceBatch | optional | batched inference grouping | owned by `engine_inference` |
