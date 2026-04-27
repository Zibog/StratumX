# Heavy Domain Observation Rate Scope And Delivery Constitution Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define observation classes, scope limits, delivery guarantees, and throttling law for heavy-domain packets.

## Observation classes
| Class | Scope law | Delivery guarantee | Drop posture | Retention posture |
|---|---|---|---|---|
| live stream | view-, focus-, or lane-scoped only | best-effort bounded stream | legal when declared and diagnosed | transient |
| event | exact entity or scope ids required | at-least-once retained event | illegal to drop silently | retained by policy |
| snapshot | explicit request or checkpoint only | complete bundle for declared scope | illegal to truncate silently | retained by artifact policy |
| certification | scenario-scoped authoritative slice | complete retained bundle | never drop silently | retained until seal decision |

## Throttling law
A family may be throttled only if:
- the class is declared throttle-legal;
- the downgrade is surfaced to consumers;
- the first throttle reason is published;
- the family remains audit-safe for its current use.

## Consumer table law
Every observation family must list primary consumers in editor, tooling, and comparison surfaces.

## Required consumer classes
- viewport and cockpit
- focused inspector
- lab replay / compare
- certification runner
- release-seal review

## Failure families
- `OBS_SCOPE_*`
- `OBS_RATE_*`
- `OBS_DROP_*`
- `OBS_RETENTION_*`
