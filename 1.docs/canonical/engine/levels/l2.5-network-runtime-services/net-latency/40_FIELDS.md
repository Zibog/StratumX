# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| latency_estimate | LatencyEstimate | required | round-trip time semantic | owned by `engine_net_latency` |
| latency_bucket | LatencyBucket | required | latency classification bucket | owned by `engine_net_latency` |
| jitter_measure | JitterMeasure | required | packet timing variance | owned by `engine_net_latency` |
| latency_compensation | LatencyCompensation | optional | client-side prediction parameters | owned by `engine_net_latency` |
