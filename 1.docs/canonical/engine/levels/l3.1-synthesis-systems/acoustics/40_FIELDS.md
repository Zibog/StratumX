# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| acoustic_source | AcousticSource | required | sound source descriptor | owned by `engine_acoustics` |
| acoustic_environment | AcousticEnvironment | required | environment acoustic properties | owned by `engine_acoustics` |
| acoustic_propagation | AcousticPropagation | required | propagation path semantic | owned by `engine_acoustics` |
| acoustic_output | AcousticOutput | required | synthesized audio frame | owned by `engine_acoustics` |
