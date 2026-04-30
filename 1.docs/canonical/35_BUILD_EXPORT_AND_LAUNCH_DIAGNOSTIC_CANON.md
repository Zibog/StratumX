# Build Export And Launch Diagnostic Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze the release-side diagnostic law for transforming a certified project and world state into an exported, launched, and first-result-verified product.
Build/export/launch is part of the product conveyor, not an afterthought tacked onto simulation.

## Release chain
`freeze-reviewed state -> build package -> export target -> launch build -> verify first result -> inspect blockers or sign off`

## Exact release controls
| Button | Route id | Required artifacts | Result lifecycle | Mandatory diagnostics | Deny reasons |
|---|---|---|---|---|---|
| `btn.build.package` | `route.build.package` | staging manifest, executable ref, world/package identity refs, retained baseline pointer, certification bundle backlink | ack → progress → partial → terminal | active build profile, included world ids, blocked packs, fallback posture | missing target profile, unresolved certification blocker, world identity drift |
| `btn.export.target` | `route.export.target` | export artifact, checksum, release bundle backlink, target profile ref | ack → progress → partial → terminal | export schema/profile, target compatibility verdict, packaged world/save posture | schema mismatch, export profile illegal, restore posture unresolved |
| `btn.launch.verify_first_result` | `route.launch.verify_first_result` | launch trace, first-result capture, blocker board snapshot, restore-safe baseline pointer, certified bundle digest | ack → progress → partial → terminal | first-result signature, first blocker family, first legal recovery action, device/runtime profile, bundle continuity verdict | executable missing, blocker open, launch verification scope illegal, certified bundle mismatch |

## First-result law
Launch verification must publish:
- the first visible/audible/result signature expected by the certified bundle;
- the first failure family if verification fails;
- the first legal recovery action;
- whether the launched build still matches the certified bundle;
- whether restore-safe state can still be loaded without identity drift.

## World and persistence law
A build/export/launch route is not closed if it ignores the world family.
Release diagnostics must make visible:
- which world packages were bundled;
- whether their namespaces and save/restore contracts remained lawful;
- whether chunk/material/environment bindings match the certified state;
- whether a degraded runtime posture changed between certification and launch.

## Full-game conveyor law
Release closure is not limited to terrain/material proof.
A product build must remain traceable across:
- gameplay/systemic state families participating in the proof route;
- audio zones/mix and presentation/runtime profiles;
- persistence and restore anchors;
- old-floor fallback posture if certification included it.

## Prohibitions
- No release signoff may rely on a build that cannot be tied back to a certification bundle.
- No launch success may hide a world/save mismatch, degraded posture change, or first blocker family.
- No exported artifact may be treated as verified until first-result proof is captured.
