# L4 Synchronization Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

Engine L4 must publish startup-bound public contract surfaces.
L5 synchronizes only with those public surfaces:
- ingress packet sinks
- ingress control sinks
- observation batch sources
- metric batch sources
- compatibility fact snapshot tables
- handle and ref snapshot tables
- artifact ref snapshot tables
- bridge epoch and invalidation signals

L5 does not crawl engine internals. It binds once to public export surfaces, resolves bridge epochs through declared invalidation signals, and keeps the bridge narrow.
