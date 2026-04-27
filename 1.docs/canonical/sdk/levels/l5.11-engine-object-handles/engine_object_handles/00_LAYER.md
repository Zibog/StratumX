# Engine Object Handles Layer

## Purpose
Local layer contract for `engine_object_handles`.

## Scope
- owns only `engine_object_handles` records
- resolves only the listed dependencies
- publishes only the listed sync surfaces

## Audit use
A reviewer should be able to diff this file against neighboring levels and see why `engine_object_handles` cannot be merged with them.
