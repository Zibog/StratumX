# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| connection_handle | ConnectionHandle | required | stable connection reference | owned by `engine_net_transport` |
| packet_descriptor | PacketDescriptor | required | packet header semantic | owned by `engine_net_transport` |
| transport_session | TransportSession | required | session state binding | owned by `engine_net_transport` |
| packet_lane | PacketLane | required | priority lane for packet dispatch | owned by `engine_net_transport` |
