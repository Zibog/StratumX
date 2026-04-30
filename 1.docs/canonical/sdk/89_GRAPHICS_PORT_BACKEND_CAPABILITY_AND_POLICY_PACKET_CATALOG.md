# Graphics Port Backend Capability And Policy Packet Catalog

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define SDK packets for backend registry, policy resolution, feature tiers, and backend status.

## Packet families
| Packet | Producer | Consumers | Purpose |
|---|---|---|---|
| `packet.graphics.backend_registry.v1` | engine graphics port | tooling, editor, quality | list compiled/registered backends |
| `packet.graphics.backend_policy_request.v1` | editor/tooling/app | engine graphics port | request auto/forced/headless/benchmark backend |
| `packet.graphics.backend_policy_result.v1` | engine graphics port | editor, tooling, quality | selected backend, reason, fallback chain |
| `packet.graphics.backend_caps.v1` | backend driver | editor, tooling, capture | limits, targets, feature tiers |
| `packet.graphics.backend_status.v1` | backend driver | editor, tooling | available/stubbed/failed/disabled state |
| `packet.graphics.feature_tier.v1` | engine graphics port | editor, certification | active tier and disabled features |

## Required fields
Every backend caps packet contains:
- backend class;
- backend status;
- shader target set;
- feature tier max;
- active feature tier;
- present support;
- capture support;
- optional accelerators;
- first blocker;
- fallback chain;
- platform legality.

## Compatibility law
Adding a backend class is additive.
Changing meaning of an existing feature tier is breaking.
Changing blocker code semantics is breaking.

## Current posture
`document_gold / packet_catalog_defined / implementation_open`


---
# V32 SDK Closure: Backend Capability Packets

## Required packets
| Packet | Direction | Purpose |
|---|---|---|
| `packet.graphics.backend_caps.v1` | engine -> editor/tooling | backend class/status/caps/limits |
| `packet.graphics.backend_policy_request.v1` | editor/tooling -> engine | request auto/forced/benchmark/null policy |
| `packet.graphics.backend_policy_resolution.v1` | engine -> editor/tooling | selected backend, fallback chain, blockers |
| `packet.graphics.optional_feature_verdicts.v1` | engine -> editor/tooling | optional feature status by backend |
| `packet.graphics.backend_doctor_summary.v1` | tooling -> editor | backend doctor result |

## Required fields
`backend_caps` must include backend class, backend status, first blocker code, feature tier, shader targets, presentation caps, capture caps, limits, optional feature verdicts, `is_stub`, `is_headless`, and platform legality.
