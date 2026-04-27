# Compatibility Versions Layer

## Purpose
Local layer contract for `compat_versions`.

## Scope
- owns only `compat_versions` records
- resolves only the listed dependencies
- publishes only the listed sync surfaces

## Audit use
A reviewer should be able to diff this file against neighboring levels and see why `compat_versions` cannot be merged with them.
