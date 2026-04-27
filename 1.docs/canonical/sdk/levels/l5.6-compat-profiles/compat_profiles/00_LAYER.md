# Compatibility Profiles Layer

## Purpose
Local layer contract for `compat_profiles`.

## Scope
- owns only `compat_profiles` records
- resolves only the listed dependencies
- publishes only the listed sync surfaces

## Audit use
A reviewer should be able to diff this file against neighboring levels and see why `compat_profiles` cannot be merged with them.
