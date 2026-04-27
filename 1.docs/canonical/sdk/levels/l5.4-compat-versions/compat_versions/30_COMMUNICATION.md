# Compatibility Versions Local Communication

## Sends or publishes
- publishes immutable version rows consumed by packets, controls, and metrics
- never emits mutable runtime state

## Receives or resolves
- version registry publish surface
- version lookup surface

## Local law
The communication contour above is exhaustive for `compat_versions`.
