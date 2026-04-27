# Fields

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| transfer_descriptor | TransferDescriptor | required | transfer operation semantic | owned by `engine_transfer_control` |
| transfer_policy | TransferPolicy | required | transfer budget and priority policy | owned by `engine_transfer_control` |
| transfer_batch | TransferBatch | required | grouped transfer operations | owned by `engine_transfer_control` |
| transfer_fence | TransferFence | optional | synchronization fence for transfer completion | owned by `engine_transfer_control` |
