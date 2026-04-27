# Engine Artifact Refs Layer

## Purpose
Local layer contract for `engine_artifact_refs`.

## Scope
- owns only `engine_artifact_refs` records
- resolves only the listed dependencies
- publishes only the listed sync surfaces

## Audit use
A reviewer should be able to diff this file against neighboring levels and see why `engine_artifact_refs` cannot be merged with them.
