# Operator Chain Certify Freeze And Evidence Route Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
This file defines one no-leak operator chain for canonical authoring and certification:
`btn -> route -> packet -> truth -> diagnostics -> capture -> compare -> certify -> freeze -> build/export/launch`.
It exists to stop ad-hoc shortcuts between layers.

## Chain table
| Step | Primary law | Required outputs | Blockers | Next legal action |
|---|---|---|---|---|
| btn | root `74`, editor `110` | button id, owner lab, disabled reasons, declared route | button missing, owner missing, disabled hard-stop | lower only through tooling |
| route | tooling `48–55`, `81` | normalized intent, transaction state, rollback anchor, retry budget | no legal route, invalidation conflict, cache ownership gap | emit sdk command or denial |
| packet | sdk `47–57`, `77` | command payload, ack/progress/result policy, evidence duty | unnamed payload, incompatible schema, missing result lifecycle | deliver to engine owner only |
| truth | engine owners named by the active row | legal mutation or legal read, failure family, replay window | illegal mutation, pressure overrun, missing coupling boundary | publish diagnostics and terminal result |
| diagnostics | root `75`, sdk `71`, tooling `58`, editor `100` | observation family, overlay target, focus target, artifact refs | missing observation mapping, missing focus target | capture and compare |
| capture/compare | root `67`, `68`, `75`, sdk `76`, editor `103`, `105` | baseline/failed/recovery triplet, compare mode, evidence append rule | missing baseline, compare incomplete | certify or recover |
| certify/freeze | root `68–71`, editor `87`, `109` | certification result, retained bundles, freeze posture, waiver/exemption state | unresolved blocker, old-floor failure, missing evidence | build/export/launch or deny |
| build/export/launch | root `24`, `34`, `35`, `39`; editor `82` | executable ref, launch trace, first-result verification, relay pack id | missing executable, launch blocker, first-result mismatch | recover freeze posture or repair local blocker |
| regression/scale review | root `69`, `71`, `81`, `83`; editor `81`, `89` | region validation posture, mixed-pack regression state, scale-safe declaration or denial | missing regression result, undocumented fallback, region validation gap | loop back to certify/freeze with retained blockers |

## No-leak law
- no button may bypass `editor/110`;
- no route may bypass tooling mediation;
- no sdk family may omit the lifecycle declared in `sdk/77`;
- no failure may be shown without one next legal recovery action and one focus target;
- no freeze or launch may proceed without retained artifacts, compare triplet, blocker trace, and first-result verification;
- no regression or scale review may invent a second pack vocabulary outside root `81`.
