# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| stream_descriptor | StreamDescriptor | required | streaming channel semantic | owned by `engine_stream_control` |
| stream_policy | StreamPolicy | required | bandwidth/timing policy binding | owned by `engine_stream_control` |
| stream_buffer | StreamBuffer | required | buffered frame data semantic | owned by `engine_stream_control` |
| stream_lane | StreamLane | required | priority lane for streaming dispatch | owned by `engine_stream_control` |
