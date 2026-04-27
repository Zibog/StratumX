# Living Stack Shared State And Persistence Constitution Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define shared state carriers for needs, schedules, squad intent, ecology, semantic memory boundaries, and explanation anchors.

## Shared carriers
| Carrier | Scope | Must survive save/restore |
|---|---|---|
| `living.need_state` | agent | yes |
| `living.schedule_state` | agent | yes |
| `living.social_memory` | agent/group | yes |
| `living.squad_intent` | squad | yes |
| `living.ecology_front` | pack/herd/region | yes |
| `living.dialogue_memory_window` | agent | bounded yes |
| `living.explanation_anchor` | all living families | yes |

## Boundary law
Generated prose may summarize state.
It may not become the persisted truth instead of the state carriers above.
