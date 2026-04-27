# Simulate Play Diagnostics Fix And Freeze Route Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the only lawful loop for turning authored content into a playable, diagnosable, comparable, recoverable, and freeze-safe state.
This route applies to physics, world, gameplay, graphics, audio, and cross-domain product proof equally.

## Canonical loop
`simulate/play -> inspect/reason -> compare -> capture -> recover or fix -> replay -> certify -> freeze or deny`

## Exact stage law
| Stage | Required controls | Mandatory retained outputs | Closure rule |
|---|---|---|---|
| Simulate / play | `btn.sim.play_slice` plus owning domain simulate rows in `editor/110` | trace ref, progress, active proof scope, runtime posture snapshot | simulate is illegal if scope, world identity, or baseline family is ambiguous |
| Inspect / reason | `btn.inspect.reason_chain`, `btn.trace.reason.inspect`, `btn.trace.failure.overlay`, plus owning inspect rows | overlay id, first blocker family, owning route boundary, focus target | reason inspection must identify the first failing or degraded boundary, not just a symptom |
| Compare | `btn.compare.triplet` plus owning compare rows | compare digest, baseline pointer, failed-run id, recovered-run id, pack continuity verdict | compare is mandatory before freeze-relevant recovery or release decisions |
| Capture | `btn.capture.evidence`, `btn.trace.capture.bundle`, plus owning capture rows | artifact refs, trace refs, evidence append, proof bundle lineage | capture must remain attached to the same proof scope and identity family |
| Recover / fix | `btn.recover.drop_one_rung` plus owning recover or fix rows | explicit recovery action id, new focus target, preserved lineage, changed degrade rung if any | recovery may not destroy the failed-run evidence family |
| Replay | same simulate rows as the triggering route | replay verdict against the preserved baseline family | a fix is not accepted until it is replayed against the same scope |
| Certify / freeze review | `btn.certify.pack`, `btn.freeze.review_release_blockers`, `btn.freeze.signoff_release_bundle` | certification bundle, blocker board, freeze verdict, release posture | certification is illegal if replay, baseline, or old-floor posture are unresolved |

## Cross-domain closure law
The loop is not closed unless the same proof route can reveal and retain blockers for:
- world identity or persistence;
- physics/material consequence;
- gameplay legality;
- graphics frame/posture;
- audio audibility/mix/posture;
- release continuity.

## Baseline triad law
Freeze-relevant proof must preserve three distinct identities:
- last-good baseline;
- failed-run state;
- recovered-run state.

A single overwritten snapshot is never enough.

## Fix vs degrade law
When a route is recovered through a lower rung instead of a full fix, the loop must publish:
- the active degrade rung;
- which quality/feature scope changed;
- whether the degraded result remains valid for certification or only for temporary continuation.

## Phase-closure law
- the simulate/fix loop is not closed unless the same proof scope can be replayed from saved state and from retained baseline;
- compare and capture must retain baseline, failed-run, and recovery-run artifacts as three distinct identities;
- freeze is illegal if build/export/launch would consume a state that has not passed compare, evidence, restore, and old-floor review;
- no “it works on my machine” proof may substitute for retained compare and capture evidence.
