# Button Lowering Focus And Recovery Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This file defines the exact lowering, focus, and recovery law for operator-visible button paths.

## Button-class routing table
| Button class | Lowering rule | Route state machine | Retry budget | Rollback anchor | Focus after success | Focus after retryable failure | Focus after terminal failure | Recovery action mapping | Retained artifact policy |
|---|---|---|---:|---|---|---|---|---|---|
| hydrology restore | lower to tooling `48` only | heavy compare + restore machine | 1 | restore anchor | editor `103` then `105` | editor `66` | editor `100` | restore anchor -> rerun triplet compare | baseline/failed/recovery bundle required |
| fire/weather verify | lower to tooling `48` only | coupling verify machine | 1 | previous volumetric rung | editor `103` | editor `67` | editor `100` | drop rung -> rerun band compare | failed-run capture required |
| society/tactics/ecology | lower to tooling `49` only | legality/graph/route machine | 1 / 0 / 1 by route | legality anchor / cover graph / route tier | editor `105` | owning lab | editor `100` | restore anchor or rollback graph or lower route tier | compare digest required |
| wound review | lower to tooling `50` only | replay review machine | 1 | wound baseline anchor | editor `105` | editor `72` | editor `100` | restore wound baseline -> rerun replay compare | replay triplet required |
| semantic audit | lower to tooling `51` only | guard and consequence machine | 1 | semantic baseline anchor | editor `103` then `105` | editor `73` | editor `100` | restore semantic baseline -> rerun chain compare | chain artifact required |
| floor / regression | lower to tooling `52` or `55` only | certification/replay machine | 1 / 1 | last-good baseline / restore anchor | editor `109` | editor `81` or `77` | editor `103` | recover one axis or restore anchor | failed-run and recovery-run bundles required |

## Lowering law
- no button may lower directly to sdk or engine;
- button classes may not invent new route states outside the owning route canon;
- success and failure focus targets must agree with editor `110` and sdk `77`.

## Recovery law
- every retryable failure must carry one route-local recovery action and one focus target;
- every terminal failure must carry one retained artifact ref or a hard denial code explaining why none exists;
- retained artifact policy is authoritative for certification and freeze surfaces.

## Workspace, extension, and assistant lowering note
Buttons added in editor `110` for workspace stage switching, layout save/load/reset, viewport splits, extension mounting, and assistant apply/revert must lower through the same canonical lowering mesh:
- command id
- route id
- focus rule
- rollback anchor when recoverable
- retained evidence posture when applicable
