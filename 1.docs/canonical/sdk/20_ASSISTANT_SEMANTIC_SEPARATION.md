# 20 ASSISTANT SEMANTIC SEPARATION

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Scope
This law keeps assistant/runtime/planner semantics above the bridge.

## Bridge-visible assistant-adjacent facts
`L5` may publish only neutral facts that an assistant could consume:
- ids and refs to engine/runtime subjects;
- immutable snapshots, observations, metrics, and legality verdicts;
- artifact refs and reproducible cursors/epochs.

## Bridge-forbidden assistant semantics
`L5` may never publish or own:
- natural-language prompts or completions;
- plan state, goal decomposition, or proposal ranking;
- assistant approval state;
- rewrite/apply/revert workflow ownership;
- tool recommendation policy.

## Separation rule
If a datum would change meaning when interpreted by a human/assistant rather than by a deterministic bridge consumer, it belongs above `L5`.
`L5` carries machine-checkable facts only.

## Audit checks
- assistant-facing layers can reconstruct context from `L5` facts without `L5` storing assistant intent;
- no `L5` field requires language-model semantics to interpret;
- no prompt/planner vocabulary leaks into level or registry docs.
