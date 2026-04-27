# Packet Lanes

## Role

Canonical transport lane partition.

## Lane Matrix

| Lane class | Ordering/reliability | Ceiling source | Failure posture | Illegal posture |
|---|---|---|---|---|
| control lane | reliable ordered | transport/network ceilings | refuse nonessential control chatter first | promote bulk state into control lane |
| session data lane | reliable ordered bounded | transport/network ceilings | backpressure bounded bulk first | unbounded asset/session flood |
| state/input lane | unreliable unordered bounded | transport/network ceilings | drop newest low-value packets first | hidden reliable widening |
| diagnostics lane | explicit and bounded only | diagnostics law | drop diagnostics first | stealth diagnostics lane |

## Rule

Lane classification is canonical and may not be rewritten by higher layers at runtime.

## Failure Posture

When transport pressure rises, diagnostics lanes and low-value state lanes drop first. Control and required session lanes keep their bounded guarantees until the canonical ceiling is crossed.

## Illegal Patterns

- lane reclassification by higher layers;
- reliability promotion of state/input lanes;
- hidden bulk transfer inside control lane.

## Lane Ownership Matrix

| Lane class | Owner | Illegal owner |
|---|---|---|
| control | `engine_net_transport` | gameplay or runtime shell |
| session data | `engine_net_transport` | content or transfer service |
| state/input | `engine_net_transport` | simulation family |
| diagnostics | diagnostics law only | hidden debug lane |

## Operational Contract

Lane legality is a local operating law.
No caller may relabel reliability, ordering, or backlog class after bind.
