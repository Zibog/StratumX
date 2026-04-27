# L7A Assistant Brain Model

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

`L7A` is the assistant planner/brain.
It is cold, bounded, and never becomes direct apply authority.
It normalizes goals, builds plan IR, reasons against canon, explores generation and optimization paths, plans migrations, and chooses model routing.

## L7A owns
- prompt understanding and goal normalization
- plan IR construction and sequencing
- canon reasoning outputs
- generation planning
- optimization alternatives and trade-offs
- migration plans
- model routing policy

## L7A never owns
- editor truth
- transaction ownership
- direct apply authority
- direct engine access
- hidden mutable runtime mirrors of `L6`

Allowed outputs are plan bundles, evidence requests, proposal intents, routing decisions, optimization alternatives, and migration plans.
