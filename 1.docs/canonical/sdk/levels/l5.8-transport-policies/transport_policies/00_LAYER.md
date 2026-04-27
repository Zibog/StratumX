# Transport Policies Layer

## Purpose
Local layer contract for `transport_policies`.

## Scope
- owns only `transport_policies` records
- resolves only the listed dependencies
- publishes only the listed sync surfaces

## Audit use
A reviewer should be able to diff this file against neighboring levels and see why `transport_policies` cannot be merged with them.
