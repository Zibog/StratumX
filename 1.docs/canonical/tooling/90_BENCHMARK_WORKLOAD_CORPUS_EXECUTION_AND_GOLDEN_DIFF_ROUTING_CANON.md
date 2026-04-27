# Benchmark Workload Corpus Execution And Golden Diff Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define exact routing for dream-scene benchmark packs and golden comparisons.

## Required route stages
`pack_select -> workload_bootstrap -> baseline_resolve -> execution -> threshold_eval -> compare -> evidence_append -> freeze_review_handoff`

## Mandatory corpus rows
- old-PC photoreal tunnel firefight;
- distant storm visibility and arrival;
- fire/rain/smoke coupling;
- dense urban block destruction;
- hydrology persistence restore;
- squad tactics under destruction;
- fur/cloth/wind stress;
- projectile/wound/cover chain.

## Golden diff artifact law
A successful route must append:
- baseline ref;
- run ref;
- divergence report;
- degradation digest;
- operator verdict;
- freeze handoff ref when certification posture is requested.

A green frame-time trace without the artifacts above is not a pass.

## Failure families
| Failure family | Meaning | Required recovery target |
|---|---|---|
| `BENCH_BOOT_*` | workload bootstrap failed | benchmark launcher |
| `BENCH_DIFF_*` | golden compare failed | compare review pane |
| `BENCH_LEG_*` | domain legality failed | owning lab + compare review |
| `BENCH_FLR_*` | old-floor rung visibility or cap failed | hardware floor lab |
| `BENCH_ART_*` | retained artifact missing or malformed | evidence review pane |
