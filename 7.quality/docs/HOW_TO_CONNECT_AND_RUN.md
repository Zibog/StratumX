# HOW TO CONNECT AND RUN

## 1. Подключить новые workspace members
Открой корневой `Cargo.toml` и проверь, что в `[workspace].members` есть ровно эти quality members:

```toml
"7.quality/support/stratumx_test_support",
"7.quality/suites/engine_canon_matrix",
"7.quality/suites/engine_perf_harness",
"7.quality/suites/sdk_canon_matrix",
"7.quality/suites/tooling_canon_matrix",
"7.quality/suites/editor_canon_matrix",
"7.quality/suites/engine_sdk_link_matrix",
"7.quality/suites/sdk_tooling_link_matrix",
"7.quality/tasks/stratumx_quality_tasks",
```

И убери старые ссылки на:
- `7.tests/...`
- `scripts/...`
- `artifacts/...`

## 2. Проверить app members
В этом архиве уже исправлены app paths на текущие реальные директории:
- `6.apps/engine/stratumx_engine_headless_app`
- `6.apps/engine/stratumx_engine_realtime_app`
- `6.apps/stack/stratumx_stack_runtime_app`
- `6.apps/editor/stratumx_editor_app`
- `6.apps/stack/stratumx_stack_utility`

## 3. Установить `.gitignore`
В архив уже добавлено:
- `target/`
- `7.quality/data/generated/`

## 4. Базовый прогон
Из корня репозитория:

```bash
cargo run -p stratumx_quality_tasks -- verify
cargo run -p stratumx_quality_tasks -- smoke
cargo run -p stratumx_quality_tasks -- full
cargo run -p stratumx_quality_tasks -- bench
cargo run -p stratumx_quality_tasks -- metrics
cargo run -p stratumx_quality_tasks -- evidence
cargo run -p stratumx_quality_tasks -- gold
```

## 5. Прямой прогон suite crates
Если нужно гонять пакетно:

```bash
cargo test -p engine_canon_matrix -- --nocapture
cargo test -p engine_perf_harness -- --nocapture
cargo test -p sdk_canon_matrix -- --nocapture
cargo test -p tooling_canon_matrix -- --nocapture
cargo test -p editor_canon_matrix -- --nocapture
cargo test -p engine_sdk_link_matrix -- --nocapture
cargo test -p sdk_tooling_link_matrix -- --nocapture
```

## 6. Что уже переделано в архиве
- `7.tests` -> `7.quality/suites`
- `scripts` -> `7.quality/tasks/stratumx_quality_tasks`
- `artifacts` -> `7.quality/data/baselines/imported_previous_run`
- imported local crate tests from `7.tests/engine/from_2.engine` -> `7.quality/data/fixtures/engine_imported`
- suite imports `stratumx_sdk` / `stratumx_tooling` / `stratumx_editor` переведены на единый support crate `stratumx_test_support`
- engine suite cargo paths перепривязаны на реальные текущие `2.engine/*` directories
