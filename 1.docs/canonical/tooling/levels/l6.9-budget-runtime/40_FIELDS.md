# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| budget_scope_id | BudgetScopeId | stable identity of one budget scope | unique per domain/surface |
| resource_class | ResourceClass | cpu/memory/gpu/disk class | finite enum only |
| hard_limit | HardLimit | hard stop for the scope | must be explicit |
| soft_limit | SoftLimit | degrade threshold before hard stop | must be less than or equal to hard limit |
| pressure_state | PressureState | nominal/warn/deny/defer state | must drive declared degradation only |

## Field law
All owned records above must be sufficient to reconstruct the public meaning of `budget_runtime` without consulting a hidden mirror.
