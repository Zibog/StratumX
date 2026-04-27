# Engine Runtime Handles Layer

## Purpose
Local layer contract for `engine_runtime_handles`.

## Scope
- owns only `engine_runtime_handles` records
- resolves only the listed dependencies
- publishes only the listed sync surfaces

## Audit use
A reviewer should be able to diff this file against neighboring levels and see why `engine_runtime_handles` cannot be merged with them.
