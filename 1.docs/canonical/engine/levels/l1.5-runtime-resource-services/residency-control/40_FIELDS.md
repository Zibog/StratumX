# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| residency_state | ResidencyState | required | resident/streaming/transient semantic | owned by `engine_residency_control` |
| residency_policy | ResidencyPolicy | required | policy binding for residency transitions | owned by `engine_residency_control` |
| residency_hint | ResidencyHint | optional | client-provided residency preference | owned by client, validated by `engine_residency_control` |
| pack_handle | PackHandle | required | stable resource pack reference | owned by `engine_residency_control` |
