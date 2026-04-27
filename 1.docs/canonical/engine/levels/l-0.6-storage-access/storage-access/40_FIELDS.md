# Fields

## Field Definitions

| Field Name | Semantic Type | Required/Optional | Invariant | Ownership |
|------------|---------------|-------------------|-----------|-----------|
| read_view | Immutable storage view | Required | Deterministic traversal over structured storage | engine_storage_access |
| write_window | Mutable access window | Required | Time-bounded and legality-bounded | engine_storage_access |
| access_descriptor | Access legality descriptor | Required | Bound to write window | engine_storage_access |
| access_mode | Mode classification | Required | Read, write, staged, or mixed under explicit constraints | engine_storage_access |
| traversal_plan_id | Compiled plan identifier | Required | Must match descriptor/cache entry | engine_storage_access |
| access_legality | Read/Write legality | Required | Explicit in type or descriptor | engine_storage_access |
| locality_class | Locality classification | Required | Frozen at bind | engine_storage_access |
| scratch_class | Scratch ownership class | Required | Frozen at bind | engine_storage_access |
| mutation_handoff | Mutation staging mode | Required | Staged only, no direct write entry | engine_storage_access |

## Invariant Rules

- Write windows must be bound to access descriptor, traversal plan legality, and runtime scheduling legality
- Unregistered mutable bypasses are illegal
- Traversal entry may not allocate new plan on cache hit
- Access rights must not widen after bind
- Locality class must not widen after bind
- Scratch ownership must not swap after bind
- Partition boundaries must not be rewritten during iteration
- Traversal entry is a legality checkpoint, not a convenience helper
