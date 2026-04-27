# Link Ingress Controls Layer

## Purpose
Local layer contract for `link_ingress_controls`.

## Scope
- owns only `link_ingress_controls` records
- resolves only the listed dependencies
- publishes only the listed sync surfaces

## Audit use
A reviewer should be able to diff this file against neighboring levels and see why `link_ingress_controls` cannot be merged with them.
