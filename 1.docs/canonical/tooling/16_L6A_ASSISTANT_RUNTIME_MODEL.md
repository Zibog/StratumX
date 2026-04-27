# L6A Assistant Runtime Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

`L6A` owns assistant runtime mediation only.
It packages evidence, requests models, stages proposals, lowers legal work into `L6` commands, runs safety gates, and applies or reverts only through `L6` transactions.

## L6A owns
- assistant sessions
- bounded context/evidence packs
- proposals and proposal staging
- lowering IR and lowering runtime
- safety and approval gates
- apply/revert mediation
- assistant-facing UI runtime
- bounded model request runtime

## L6A never owns
- editor truth
- direct engine access
- high-level planning truth
- hidden long-lived mirrors of mutable editor state
- silent auto-apply around safety and transaction gates

## L6A closure law
For every assistant path the canon must define:
- evidence class in
- proposal or plan class in
- lowering form
- safety verdict set
- apply/revert path
- audit evidence out
