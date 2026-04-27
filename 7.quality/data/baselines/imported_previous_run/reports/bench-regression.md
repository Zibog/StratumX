# Bench Regression Status

- Benchmarks compared: **37**
- Regressions >15%: **0**
- Baseline created during this run: **no**
- Baseline extended for new benches during this run: **no**

| Benchmark | Current | Baseline | Delta % | Status |
|---|---:|---:|---:|---|
| engine_acoustics_synthesize | 73775.810 | 73879.896 | -0.14 | ok |
| engine_agents_simulate | 61387.440 | 62289.331 | -1.45 | ok |
| engine_content_ingest | 123312.408 | 127125.675 | -3.00 | ok |
| engine_core_hot_type_ops | 2571.426 | 2626.222 | -2.09 | ok |
| engine_ecs_facade | 39054.263 | 39837.972 | -1.97 | ok |
| engine_ecs_query_execute | 41706.041 | 42415.195 | -1.67 | ok |
| engine_ecs_registry_membership | 146255.930 | 147684.926 | -0.97 | ok |
| engine_field_simulate | 61823.658 | 64024.500 | -3.44 | ok |
| engine_generation_generate | 76385.532 | 79975.166 | -4.49 | ok |
| engine_handle_validate | 867.013 | 885.682 | -2.11 | ok |
| engine_identity_issue_retire | 84168.110 | 85690.427 | -1.78 | ok |
| engine_imaging_render | 73490.009 | 73561.335 | -0.10 | ok |
| engine_inference_infer | 170004.965 | 177465.284 | -4.20 | ok |
| engine_kinetics_simulate | 62612.839 | 65439.420 | -4.32 | ok |
| engine_material_lookup | 54683.644 | 54906.848 | -0.41 | ok |
| engine_memory_control_cycle | 2127.025 | 2256.921 | -5.76 | ok |
| engine_net_latency_metrics | 38041.707 | 40812.772 | -6.79 | ok |
| engine_net_sync_snapshot | 86693.042 | 91229.737 | -4.97 | ok |
| engine_net_transport_send | 152918.245 | 157175.049 | -2.71 | ok |
| engine_residency_control_metrics | 32726.366 | 34027.751 | -3.82 | ok |
| engine_runtime_headless_step | 16005.836 | 16595.757 | -3.55 | ok |
| engine_runtime_realtime_step | 16732.828 | 17321.020 | -3.40 | ok |
| engine_runtime_tick | 31945.835 | 33170.190 | -3.69 | ok |
| engine_startup_launch_plan | 6283.460 | 11275.953 | -44.28 | ok |
| engine_storage_access_bind | 29.394 | 30.985 | -5.13 | ok |
| engine_storage_layout_validate | 3894.570 | 3925.779 | -0.79 | ok |
| engine_storage_mutation_assemble | 66795.674 | 69725.621 | -4.20 | ok |
| engine_stream_control_tick | 91930.019 | 95846.013 | -4.09 | ok |
| engine_transfer_control_submit | 60622.247 | 63345.246 | -4.30 | ok |
| engine_world_region_dirty_tracking | 106220.351 | 109887.403 | -3.34 | ok |
| engine_world_snapshot_apply | 38334.666 | 41433.518 | -7.48 | ok |
| engine_world_spatial_address | 15644.415 | 15984.839 | -2.13 | ok |
| scenario_content_batch | 5513.382 | 5375.528 | 2.56 | ok |
| scenario_network_batch | 1583.705 | 1627.595 | -2.70 | ok |
| scenario_resource_pipeline_batch | 1525.606 | 1551.120 | -1.64 | ok |
| scenario_runtime_world_batch | 1293.989 | 1331.241 | -2.80 | ok |
| scenario_simulation_batch | 5955.985 | 6144.662 | -3.07 | ok |
