# Engine State Refs Layer

## Purpose
Local layer contract for `engine_state_refs`.

## Scope
- owns only `engine_state_refs` records
- resolves only the listed dependencies
- publishes only the listed sync surfaces

## Audit use
A reviewer should be able to diff this file against neighboring levels and see why `engine_state_refs` cannot be merged with them.
