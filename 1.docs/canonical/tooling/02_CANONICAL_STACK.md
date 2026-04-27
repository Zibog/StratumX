# Canonical Stack

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Execution temperatures
- **hot spine**: `L5 -> L6`
- **warm bridge**: `L6A`
- **cold control spine**: `L7` and `L7A`

## L6. Editor core / editor operating layer
- `L6.0  authority_core`
- `L6.1  command_envelopes`
- `L6.2  transaction_ledger`
- `L6.3  snapshot_plane`
- `L6.4  index_plane`
- `L6.5  derived_plane`
- `L6.6  artifact_plane`
- `L6.7  stream_plane`
- `L6.8  cache_plane`
- `L6.9  budget_runtime`
- `L6.10 workspace_runtime`
- `L6.11 validation_runtime`
- `L6.12 preview_runtime`
- `L6.13 build_runtime`
- `L6.14 release_runtime`
- sidecar tool levels: `l6.0-tool-session` through `l6.17-tool-view-refs`

## L6A. Assistant runtime / bridge
- `L6A.0 assistant_sessions`
- `L6A.1 context_evidence_packs`
- `L6A.2 proposal_runtime`
- `L6A.3 lowering_runtime`
- `L6A.4 safety_gates`
- `L6A.5 apply_revert_runtime`
- `L6A.6 assistant_ui_runtime`
- `L6A.7 model_request_runtime`

## L7. Studio orchestrator / meta layer
- `L7.0 project_orchestration`
- `L7.1 content_campaigns`
- `L7.2 world_campaigns`
- `L7.3 simulation_campaigns`
- `L7.4 release_campaigns`
- `L7.5 automation_meta`
- `L7.6 governance_meta`
- `L7.7 reporting`

## L7A. Assistant planner / brain
- `L7A.0 prompt_understanding`
- `L7A.1 planning_engine`
- `L7A.2 canon_reasoner`
- `L7A.3 generation_planner`
- `L7A.4 optimization_reasoner`
- `L7A.5 migration_planner`
- `L7A.6 model_routing`

Families compose lanes and clients. Families never create hidden truth stores when a canonical plane already exists.
