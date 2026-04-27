# Communication Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

Canonical bridge classes:
- command packet
- binding control
- execution signal
- observation record
- metric frame
- compatibility fact
- compatibility verdict
- transport policy
- legality verdict
- opaque handle
- opaque ref
- opaque artifact ref

Canonical publication forms:
- ordered ingress sink
- immutable bridge snapshot
- immutable egress batch
- monotonic reader cursor
- payload ref and anchor record

Communication law:
- every class is typed
- every class is bounded
- every class has explicit replay rules
- every class has explicit ownership rules
- every class has an explicit hot-path allocation policy
- every write-side publication enters through ordered ingress sinks only
- every read-side fanout leaves through immutable batches or immutable snapshots only

L5 never publishes raw engine memory, raw pointers, or editor-shaped structs.
