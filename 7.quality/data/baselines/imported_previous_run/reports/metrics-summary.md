# Engine Metrics Summary

| Benchmark | Mean | Median | Std Dev | Scale |
|---|---:|---:|---:|---|
| engine_acoustics_synthesize | 73.776 us | 73.192 us | 3.161 us | batch-scale operation |
| engine_agents_simulate | 61.387 us | 61.204 us | 2.393 us | batch-scale operation |
| engine_content_ingest | 123.312 us | 123.478 us | 4.880 us | batch-scale operation |
| engine_core_hot_type_ops | 2.571 us | 2.564 us | 75.879 ns | batch-scale operation |
| engine_ecs_facade | 39.054 us | 38.885 us | 873.146 ns | batch-scale operation |
| engine_ecs_query_execute | 41.706 us | 40.944 us | 4.539 us | batch-scale operation |
| engine_ecs_registry_membership | 146.256 us | 145.411 us | 4.767 us | batch-scale operation |
| engine_field_simulate | 61.824 us | 61.579 us | 1.926 us | batch-scale operation |
| engine_generation_generate | 76.386 us | 76.408 us | 2.065 us | batch-scale operation |
| engine_handle_validate | 867.013 ns | 866.017 ns | 27.039 ns | small bounded operation |
| engine_identity_issue_retire | 84.168 us | 84.145 us | 2.884 us | batch-scale operation |
| engine_imaging_render | 73.490 us | 73.442 us | 2.637 us | batch-scale operation |
| engine_inference_infer | 170.005 us | 170.139 us | 4.217 us | batch-scale operation |
| engine_kinetics_simulate | 62.613 us | 62.408 us | 2.885 us | batch-scale operation |
| engine_material_lookup | 54.684 us | 54.806 us | 1.822 us | batch-scale operation |
| engine_memory_control_cycle | 2.127 us | 2.133 us | 53.538 ns | batch-scale operation |
| engine_net_latency_metrics | 38.042 us | 37.869 us | 986.795 ns | batch-scale operation |
| engine_net_sync_snapshot | 86.693 us | 86.811 us | 2.386 us | batch-scale operation |
| engine_net_transport_send | 152.918 us | 151.346 us | 8.489 us | batch-scale operation |
| engine_residency_control_metrics | 32.726 us | 32.487 us | 1.281 us | batch-scale operation |
| engine_runtime_headless_step | 16.006 us | 16.022 us | 466.834 ns | batch-scale operation |
| engine_runtime_realtime_step | 16.733 us | 16.650 us | 1.214 us | batch-scale operation |
| engine_runtime_tick | 31.946 us | 31.886 us | 1.004 us | batch-scale operation |
| engine_startup_launch_plan | 6.283 us | 6.212 us | 224.969 ns | batch-scale operation |
| engine_storage_access_bind | 29.394 ns | 29.317 ns | 0.882 ns | micro-only; not production-shaped |
| engine_storage_layout_validate | 3.895 us | 3.876 us | 114.570 ns | batch-scale operation |
| engine_storage_mutation_assemble | 66.796 us | 66.782 us | 2.214 us | batch-scale operation |
| engine_stream_control_tick | 91.930 us | 91.603 us | 2.428 us | batch-scale operation |
| engine_transfer_control_submit | 60.622 us | 60.782 us | 2.191 us | batch-scale operation |
| engine_world_region_dirty_tracking | 106.220 us | 106.144 us | 3.173 us | batch-scale operation |
| engine_world_snapshot_apply | 38.335 us | 38.132 us | 1.436 us | batch-scale operation |
| engine_world_spatial_address | 15.644 us | 15.601 us | 425.013 ns | batch-scale operation |
| scenario_content_batch | 5.513 us | 5.464 us | 386.815 ns | batch-scale operation |
| scenario_network_batch | 1.584 us | 1.574 us | 95.158 ns | batch-scale operation |
| scenario_resource_pipeline_batch | 1.526 us | 1.512 us | 77.955 ns | batch-scale operation |
| scenario_runtime_world_batch | 1.294 us | 1.284 us | 103.772 ns | batch-scale operation |
| scenario_simulation_batch | 5.956 us | 5.865 us | 413.928 ns | batch-scale operation |
