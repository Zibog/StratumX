# 7.quality

Единый quality-layer для StratumX.

Внутри слоя:
- `support/stratumx_test_support` — общие типы, фикстуры и in-memory runtime support для suite-тестов.
- `suites/*` — canon/perf/link/editor/tooling/sdk suites.
- `tasks/stratumx_quality_tasks` — единая оркестрация verify/smoke/full/bench/metrics/evidence/gold.
- `data/fixtures` — импортированные fixture-наборы.
- `data/baselines` — перенесённые legacy outputs и baseline-данные.
- `data/generated` — runtime outputs quality-layer. Эту папку не коммитить.

Базовый запуск:
```bash
cargo run -p stratumx_quality_tasks -- verify
cargo run -p stratumx_quality_tasks -- smoke
cargo run -p stratumx_quality_tasks -- full
cargo run -p stratumx_quality_tasks -- bench
cargo run -p stratumx_quality_tasks -- gold
```
