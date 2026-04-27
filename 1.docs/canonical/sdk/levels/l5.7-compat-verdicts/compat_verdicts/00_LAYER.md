# Compatibility Verdicts Layer

## Purpose
Local layer contract for `compat_verdicts`.

## Scope
- owns only `compat_verdicts` records
- resolves only the listed dependencies
- publishes only the listed sync surfaces

## Audit use
A reviewer should be able to diff this file against neighboring levels and see why `compat_verdicts` cannot be merged with them.
