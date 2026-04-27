# Resource Discipline

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Product laws
- product surfaces are activation-bounded
- inactive suites are cold
- closed panels are cold
- hidden caches are forbidden
- equivalent data classes must co-reside by lifetime and invalidation regime
- panel state, suite state, runtime-attach presentation, and service job state must not be arbitrarily intermixed

## Concurrency laws
- UI state and focused interaction routing remain single-writer
- indexing, import/export, preview generation, validation scans, dependency analysis, bake/build/release queueing, and thumbnail generation may run in background jobs
- background jobs must be bounded, cancelable where practical, and visible to diagnostics/budget surfaces

## Memory laws
- browser and outliner projections are index-backed and partial where possible
- diagnostics and runtime streams are paged or windowed when histories grow large
- preview artifacts are discardable
- build/release artifacts are never stored in panel-local caches as truth
