# Benchmark Golden Diff And Certification Corpus Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the operational routing for scene corpus execution, golden diff, and certification verdict emission.

## Required corpora
- `corpus.visual.tunnel`
- `corpus.visual.storm`
- `corpus.substrate.fire_rain_smoke`
- `corpus.substrate.hydrology_persistence`
- `corpus.living.tactics_under_destruction`
- `corpus.surface.fur_wind_wetness`

## Route law
Execution -> packet collection -> artifact retention -> compare -> verdict -> blocker publication -> focus return.

## Diff artifact law
A golden diff artifact must contain:
- scene/workload identity,
- packet bundle refs,
- image/audio/frame artifacts when relevant,
- explicit downgrade verdicts,
- baseline/run compatibility class.
