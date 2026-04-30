# Project To Playable Product Operator Playbook Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze one full-game-first product conveyor from empty project to a launched, diagnosable, restore-safe build.
This playbook is not limited to day-zero terrain/material setup. It defines the canonical expansion from material-first world creation into a playable game contour.

## Canonical full-game conveyor
1. Bootstrap project and workspace identity.
2. Create or open world package and lock world identity/namespace.
3. Import heightmap or terrain source and establish chunk topology.
4. Sculpt, flatten, paint surface families, and bind material law.
5. Bind sky, atmosphere, time, weather, and environmental context.
6. Validate world package, chunk invalidation, material linkage, and persistence posture.
7. Import, canonicalize, and bind content/placements.
8. Author gameplay rules, population, ecology, tactics, inventory/economy, and quest/event state against the same world identity.
9. Author audio zones, emitter classes, listener profiles, mix policy, and runtime presentation/profile surfaces.
10. Simulate, inspect, compare, capture, recover, and certify using one proof-region or proof-route family.
11. Save, restore, and replay with stable identity and retained baselines.
12. Freeze, build, export, launch, and verify first-result continuity.

## Exact law per conveyor segment
| Segment | Primary controls | Required owned outputs | Mandatory cross-domain proof | First blocker family |
|---|---|---|---|---|
| project bootstrap | `btn.project.new_project`, `btn.project.open_project`, `btn.project.save_project`, `btn.project.save_project_as` | workspace id, project id, save profile id | workspace/layout identity proof | layout or identity deny |
| world package bootstrap | `btn.world.open_world`, `btn.world.save_world`, world package creation flow | world id, namespace, world manifest, environment binding refs | world identity and namespace continuity | world identity unresolved |
| terrain/material-first authoring | `btn.terrain.*`, `btn.material.*`, `btn.sky.*` base-shell rows | chunk topology, layer weights, archetype and response bindings, sky bindings | material-first world truth and save-safe chunk lineage | unresolved terrain/material truth |
| world validation and persistence | `btn.world.validate_world`, save/load chunk rows, validation routes | legality verdict, chunk invalidation status, persistence posture | restore-safe world package proof | validation or persistence deny |
| content import and placements | editor `107`, `108`, placement and content routes | canonical asset refs, placement refs, runtime binding refs | content lineage bound to the same world identity | import/binding drift |
| gameplay and systems authoring | owning rows in `editor/110` and `editor/108` | rulesets, population/ecology/tactics/economy/quest state, legality verdicts | game systems bound to the authored world and proof-region | gameplay legality deny |
| audio and presentation authoring | `btn.audio.*`, graphics/presentation rows, runtime profile routes | audio zone/mix state, listener profiles, frame/presentation profiles | world-aware audio/presentation readiness | audio/presentation deny |
| simulate/fix/certify | `btn.sim.play_slice`, compare/capture/certify rows | trace ref, compare digest, evidence bundle, certification verdict | same proof route replayed across current/failed/recovered state | baseline or blocker deny |
| save/restore/replay | timeline/save rows | save snapshot, restore digest, replay verdict | stable identity across restored build-relevant state | restore or identity deny |
| build/export/launch | `btn.build.package`, `btn.export.target`, `btn.launch.verify_first_result` | executable ref, export bundle, launch trace, first-result verification | certified bundle continuity to launched build | release blocker deny |

## Full-game-first closure law
The product conveyor is not closed if it only proves terrain/material/sky authoring.
A full playable-product claim requires the same conveyor to admit:
- world identity and save/restore continuity;
- gameplay rule and systemic-state authoring;
- audio and presentation authoring as first-class steps;
- compare/capture/recover continuity before release steps;
- launch verification against the certified bundle.

## Proof-region law
A proof-region remains the minimum lawful target for closure claims, but it must be able to host all critical product families:
- terrain/material/surface-family truth;
- environment and weather;
- population/tactics/ecology slices where relevant;
- audio zones/mix proof;
- persistence and restore proof;
- build/export/launch continuity proof.

## Forbidden shortcuts
- No full-game readiness claim may skip world-family validation.
- No product route may bolt gameplay state onto an identity-ambiguous world package.
- No audio/presentation route may be treated as postscript after “real game content” is finished.
- No build/export/launch route may consume uncertified or non-restorable state.
