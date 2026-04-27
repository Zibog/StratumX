# Full logging runbook

Run from repository root.

```powershell
New-Item -ItemType Directory -Force -Path .\7.quality\data\generated\logs | Out-Null
$env:RUST_BACKTRACE = "full"

$commands = @(
  'cargo test --workspace --all-targets -- --nocapture',
  'cargo test -p engine_canon_matrix -p engine_perf_harness -p sdk_canon_matrix -p tooling_canon_matrix -p editor_canon_matrix -p engine_sdk_link_matrix -p sdk_tooling_link_matrix -- --nocapture',
  'cargo run -p stratumx_quality_tasks -- verify',
  'cargo run -p stratumx_quality_tasks -- smoke',
  'cargo run -p stratumx_quality_tasks -- full',
  'cargo run -p stratumx_quality_tasks -- bench',
  'cargo run -p stratumx_quality_tasks -- metrics',
  'cargo run -p stratumx_quality_tasks -- evidence',
  'cargo run -p stratumx_quality_tasks -- gold'
)

$i = 1
foreach ($cmd in $commands) {
  $logPath = ".\7.quality\data\generated\logs\run-$('{0:D2}' -f $i).txt"
  Write-Host "RUN[$i] $cmd"
  cmd /c "$cmd 2>&1" | Tee-Object -FilePath $logPath
  $i++
}
```
