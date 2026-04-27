# Compatibility Capabilities Layer

## Purpose
Local layer contract for `compat_capabilities`.

## Scope
- owns only `compat_capabilities` records
- resolves only the listed dependencies
- publishes only the listed sync surfaces

## Audit use
A reviewer should be able to diff this file against neighboring levels and see why `compat_capabilities` cannot be merged with them.
