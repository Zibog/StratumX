# Forbidden Connections

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

Forbidden:
- L5 owning editor truth
- L5 owning assistant runtime truth
- L5 owning campaign or plan graphs
- L5 owning build queues or artifact manifests
- handles acting as direct mutation authority
- refs acting as direct mutation authority
- artifact refs acting as build ownership
- upper layers bypassing L5 around engine L4
- raw engine pointers or deep engine memory published through L5
- one mutable mega-store spanning snapshots, ingress lanes, and egress batches
- default per-envelope heap allocation on hot write or hot read publication paths
