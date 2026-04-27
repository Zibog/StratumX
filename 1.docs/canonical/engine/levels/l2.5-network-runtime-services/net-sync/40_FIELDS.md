# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| sync_state | SyncState | required | world state synchronization semantic | owned by `engine_net_sync` |
| sync_authority | SyncAuthority | required | authority assignment for state | owned by `engine_net_sync` |
| sync_rollback | SyncRollback | required | rollback/replay semantic | owned by `engine_net_sync` |
| sync_snapshot | SyncSnapshot | required | serialized state snapshot | owned by `engine_net_sync` |
