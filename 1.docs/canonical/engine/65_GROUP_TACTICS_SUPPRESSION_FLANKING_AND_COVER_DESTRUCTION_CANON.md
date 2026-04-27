# Group Tactics Suppression Flanking And Cover Destruction Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define runtime truth for squad plans, suppression pressure, cover graph legality, flanking windows, and destruction-aware tactic recovery.

## Exact truth objects
| Truth object | Meaning |
|---|---|
| `SquadIntentGraph` | squad role nodes, intent edges, and plan phase |
| `SuppressionField` | suppression bands, confidence loss, and movement restrictions |
| `CoverGraphState` | cover nodes, destruction state, and line-of-fire legality |
| `FlankWindowSet` | declared flank opportunities and expiry windows |
| `RollbackAnchorSet` | last-good tactics graph and cover graph anchors |
| `TacticsReplayWindow` | compare horizon for plan/recovery review |

## Exact state machine
`assembling -> committed -> suppressing / flanking -> disrupted -> rolled_back / recovered -> certified`

## Exact phase order
1. ingest threat and cover updates
2. resolve suppression
3. score flank windows
4. validate cover graph
5. publish blocker and rollback anchor

## Exact coupling boundaries
- may read wound and destruction truth but may not own them
- must publish one rollback anchor whenever cover legality changes
- may not mutate population truth directly

## Exact fail / denial families
- `tactics.cover.graph_invalid`
- `tactics.flank.window_illegal`
- `tactics.suppression.overrun`
- `tactics.rollback_anchor_missing`

## Exact resource envelope
cpu `<= 5.2 ms`; gpu overlays `<= 0.9 ms`; ram `<= 448 MiB`; disk/IO none on hot path

## Exact replay window
stable 10-second tactics replay with checkpoints on plan commit, suppression spike, cover break, and rollback

## Exact certification duties
- retain tactics graph digest, cover graph capture, flank legality board, rollback anchor manifest
- retain failed/recovery compare bundle
- retain first blocking code

## Exact legal recovery actions
- rollback to previous cover graph
- demote flank window tier
- rerun plan compare before certification

## Exact domain command families
- `verify.tactics_cover`
- `compare.plan_triplet`
- `capture.cover_break`
- `recover.tactics_anchor`



## Exact editor entrypoints
- `btn.tac.author_doctrine`
- `btn.tac.bind_cover_rule`
- `btn.tac.inspect_cover_graph`
- `btn.tac.simulate_cover_break`
- `btn.tac.compare_triplet`
- `btn.tac.capture_evidence`
- `btn.tac.recover_baseline`
- `btn.tac.certify_pack`

## Phase-3 proof slices
- squad role allocation survives suppression spikes without hidden rewrites;
- destruction-aware cover break invalidates routes and emits rollback anchors;
- every replanned tactic exports one reason fragment to `engine/76`.

## Required publications
- `event.tac.cover_or_plan_delta.v1` with `squad_id`, `cover_graph_ref`, `flank_window_set`, `trace_ref`, `artifact_ref`;
- `artifact.tac.cover.triplet` with baseline/failed/recovery and first blocker code.


## Current posture
`document_gold / doc_closed_impl_open`
