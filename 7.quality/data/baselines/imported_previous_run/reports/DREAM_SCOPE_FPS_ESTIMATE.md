# Dream-Scope FPS Estimate Model

## Important note

- This is a modeled estimate, not a measured shipped-game FPS result.
- The current engine benchmarks are useful for engine-surface sanity, but they do not measure full-scene photoreal rendering, large-world streaming, destructive terrain, cloth, fluids, wildfire propagation, or full combat AI at dream-game scale.
- Therefore the estimates below are engineering ranges under explicit assumptions, not guaranteed performance claims.

## Local metric anchors

- `engine_runtime_tick` mean: **31945.834977516894 ns**
- `engine_runtime_headless_step` mean: **16005.836146797641 ns**
- `engine_runtime_realtime_step` mean: **16732.828113119016 ns**
- `engine_world_snapshot_apply` mean: **38334.6663685717 ns**

## Interpretation

- These anchors show that the current engine micro/batch surfaces are lightweight relative to a 16.67 ms frame budget, but they do **not** represent final renderer cost or world-scale simulation cost.
- In your dream game, the dominant costs will move to GPU visibility, lighting, shadows, volumetrics, destruction, animation solve, streaming, and broadphase/update fanout under huge active sets.

## Modeled FPS ranges for the described dream game

| GPU tier | Literal dream spec, ultra visual target, 1080p | Scalable shipping high preset, 1080p | Balanced shipping preset, 1080p |
|---|---:|---:|---:|
| GTX 1070 | 8–15 FPS | 28–42 FPS | 45–60 FPS |
| GTX 1660 | 6–12 FPS | 24–38 FPS | 40–55 FPS |

## Gold-level conclusion

- **60 FPS on 10-year-old GPUs is not credible for the literal dream spec on ultra.**
- **60 FPS can become plausible only for a strongly tiered shipping preset** with strict simulation locality, aggressive streaming, multi-tier destruction fidelity, selective cloth/fluid solve, distant-weather impostors, and lighting/visibility budgets enforced by the engine.
- The current canonical engine shape is appropriate for enabling those scalability laws, but it is not yet a proof that the final dream game will sustain 60 FPS on GTX 1070 / GTX 1660.
