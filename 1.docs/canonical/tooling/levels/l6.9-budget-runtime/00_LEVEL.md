# Budget Runtime

## Role
`budget_runtime` enforces pressure policy for CPU, memory, GPU, and disk across live tooling workloads.

## Owns
- `budget_scope_id`
- `resource_class`
- `hard_limit`
- `soft_limit`
- `pressure_state`

## Consumes
- `l6.8-cache-plane`

## Emits
- pressure decisions
- defer/deny signals for preview/build/release/runtime services

## Never owns
- authority mutation truth
- hidden adaptive heuristics without declared thresholds
- editor UX ownership
