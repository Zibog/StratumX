# Operator Focus And Recovery Test Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Define the live test for success focus, failure focus, retry focus, recovery focus, and next legal action.

## Assertions
1. every promoted button has a success focus target;
2. every denial family has a visible focus target and one next legal action;
3. every retryable failure has a retry focus target and retry budget;
4. every terminal failure exposes one next legal recovery action and one blocker trace;
5. every recover action returns focus to one concrete rerun, compare, or review surface;
6. editor focus targets agree with tooling recovery mapping and sdk result lifecycle;
7. no operator is forced to infer what to do next.

## Required live evidence
- focus-target extract from `editor/110`;
- recovery mapping extract from tooling;
- lifecycle extract from sdk `77`;
- runtime monitor screenshots or trace dumps from `editor/100`;
- evidence append extracts from editor `103`, `105`, and `109`.

## Failure posture
If the operator must guess where to look next, the test fails.
If a failure state has no explicit next legal action, the test fails.
If focus jumps directly to freeze or signoff without required compare/evidence context, the test fails.

## Phase linkage
This test is mandatory for phase 1 exit and remains gating for phases 5 and 6.
