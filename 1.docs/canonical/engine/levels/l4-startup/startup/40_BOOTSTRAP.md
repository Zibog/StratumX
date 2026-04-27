# Bootstrap

## Role

Startup bootstrap envelope.

## Data Model

Bootstrap owns pre-runtime validation, profile and role admission, manifest verification, startup-mediated runtime-pack resolution, optional bounded warmup artifacts, and the handoff into runtime ownership.
Bootstrap is temporary; it may not retain long-lived ownership after legal launch.

## Bootstrap Matrix

| Item | Canonical law | Illegal posture |
|---|---|---|
| manifest verification | bounded by startup verification budget law | unbounded prelaunch scan or retry storm |
| profile admission | must satisfy hardware-floor contract | silent widening of profile floor |
| role admission | must satisfy profile and role law | runtime-service-owned role bind |
| runtime-pack resolution | startup-mediated only | upward crate dependency on `engine_content` |
| warmup artifacts | optional and budget-bounded | hidden persistent bootstrap caches |

## Canonical Law

- bootstrap owns validation and admission only until legal runtime handoff completes;
- bootstrap may refuse illegal startup requests but may not redefine runtime, network, or synthesis ownership;
- bootstrap may not retain allocator, residency, or transfer governance after launch;
- startup artifacts must remain inside startup budget, diagnostics law, and hardware-floor law.

## Illegal Patterns

- bootstrap retaining long-lived service ownership;
- validation work widened into runtime-phase execution;
- undeclared warmup artifacts or hidden retry loops.
