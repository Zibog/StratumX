# Link Egress Metrics Layer

## Purpose
Local layer contract for `link_egress_metrics`.

## Scope
- owns only `link_egress_metrics` records
- resolves only the listed dependencies
- publishes only the listed sync surfaces

## Audit use
A reviewer should be able to diff this file against neighboring levels and see why `link_egress_metrics` cannot be merged with them.
