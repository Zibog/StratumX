# Forbidden Connections

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

Forbidden:
- `L6 -> engine` direct access around `L5`
- `L6A -> engine` direct access around `L5`
- `L7 -> engine` direct access around `L5`
- `L7A -> engine` direct access around `L5`
- `L7A -> L6` direct mutation
- `L7 -> L6` hidden frame-level runtime ownership
- `L6A -> L6` authority ownership
- families owning secret truth stores
- derived planes owning truth
- cache planes owning truth
- preview systems owning truth
- build systems owning editor authority
- release systems owning editor authority
- model runtime owning plan truth or proposal truth
