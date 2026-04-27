# STRATUMX_SDK_L5_ASSISTANT_AND_GENERATION_CONSTITUTION

## Scope
This constitution isolates assistant/generation concerns from the bridge while still permitting factual support for them.

## Binding laws
- `L5` may publish only machine-checkable facts useful to assistant/generation layers;
- prompts, proposals, plans, and accepted edits are above `L5`;
- generated artifacts may be referenced by `L5` only once they are legal artifact facts, never while still proposal-state.

## Audit checks
- assistant vocabulary does not leak into bridge rows;
- generation workflows consume `L5` facts without making `L5` their owner;
- artifact references remain factual, not aspirational.
