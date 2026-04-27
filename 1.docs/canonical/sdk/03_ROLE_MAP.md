# Role Map

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Write-side roles
- ingress packets: mutation and request envelopes
- ingress controls: binding controls and execution signals

## Read-side roles
- observations: bounded event and status publication
- metrics: bounded counters, gauges, timings, and pressure frames

## Fact roles
- versions: compatibility tuples and protocol versions
- capabilities: legal feature availability facts
- profiles: runtime profile facts and cost envelopes

## Gate roles
- verdicts: compatibility decisions derived from facts
- transport policies: admissibility and publication constraints
- legality gates: mutation legality decisions

## Opaque export roles
- session handles: scoped runtime session tokens
- object handles: scoped object tokens
- runtime handles: scoped runtime service tokens
- identity refs: immutable identity projections
- state refs: immutable or monotonic state projections
- artifact refs: immutable generated artifact projections
