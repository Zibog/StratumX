# Generative Dialogue And Runtime Semantic Consequence Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define runtime truth for guarded dialogue intent, semantic policy, consequence chains, and restore-safe recovery.

## Exact truth objects
| Truth object | Meaning |
|---|---|
| `DialogueIntentLedger` | speaker, target, intent class, and turn state |
| `SemanticPolicyGateSet` | guard ids, policy tiers, and hard denials |
| `ConsequenceLedger` | world-facing semantic consequence ids and persistence class |
| `MemoryDeltaSet` | short-horizon memory deltas and compare identity |
| `BaselineGuardAnchor` | last-good semantic baseline and rollback anchor |
| `SemanticReplayWindow` | compare horizon for guarded chain review |

## Exact state machine
`pending -> guarded -> emitted -> consequences_applied -> drifted / denied -> recovered -> certified`

## Exact phase order
1. bind intent and scope
2. evaluate guards
3. emit allowed utterance class
4. apply semantic consequences
5. publish drift/denial and recovery anchor

## Exact coupling boundaries
- may reference society, quest, and persistence truth as read-only prerequisites
- may not invent gameplay authority outside declared consequence classes
- must expose one rollback anchor before any persisted semantic consequence may certify

## Exact fail / denial families
- `semantic.policy.guard_fail`
- `semantic.consequence.drift`
- `semantic.baseline.missing`
- `semantic.memory.delta_invalid`

## Exact resource envelope
cpu `<= 4.4 ms`; gpu `n/a`; ram `<= 448 MiB`; disk/IO retained chains only

## Exact replay window
stable 12-turn semantic replay with checkpoints at guard evaluation, consequence publish, drift detect, and recovery

## Exact certification duties
- retain guard board, consequence chain artifact, baseline/failure/recovery triplet, first blocker code
- retain memory delta digest and focus target id

## Exact legal recovery actions
- restore semantic baseline
- rollback one consequence chain step
- rerun guarded chain compare before certification

## Exact domain command families
- `verify.semantic_chain`
- `compare.semantic_triplet`
- `capture.guard_failure`
- `recover.semantic_anchor`



## Exact editor entrypoints
- `btn.semantic.author_grounding_policy`
- `btn.semantic.bind_world_consequence`
- `btn.semantic.inspect_denial_path`
- `btn.semantic.simulate_outcome`
- `btn.semantic.compare_outcome`
- `btn.semantic.capture_evidence`
- `btn.semantic.recover_baseline`
- `btn.semantic.certify_pack`

## Phase-3 proof slices
- guarded generation may deny or emit but may not invent authority;
- world consequence binding survives replay and restore-safe compare;
- every emitted or denied outcome exports one reason fragment to `engine/76`.

## Required publications
- `event.semantic.outcome_applied.v1` with `speaker_id`, `guard_result`, `consequence_ref`, `trace_ref`, `artifact_ref`;
- `artifact.semantic.guard.triplet` with baseline/failed/recovery and memory-delta digest.


## Current posture
`document_gold / doc_closed_impl_open`
