# STRATUMX_MEMORY_GPU_DISK_CONSTITUTION

## Scope
This constitution governs storage and resource-discipline across tooling.

## Binding laws
- memory, GPU, and disk products must reside in declared planes or artifact stores;
- caches are bounded and evictable;
- heavy outputs are reproducible through artifact/build/release law rather than hidden temp state.

## Audit checks
- cache/build/artifact/runtime docs declare rebuild and eviction posture;
- no background service accumulates unsurfaced permanent state;
- resource budgets remain compatible with hot/cold activation law.
