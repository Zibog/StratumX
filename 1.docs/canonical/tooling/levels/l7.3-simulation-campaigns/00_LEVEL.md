# Simulation Campaigns

## Role
`simulation_campaigns` groups simulation/AI balancing campaigns and verification milestones.

## Owns
- `simulation_campaigns_record_id`
- `scope_id`
- `state`
- `input_ref_set`
- `output_ref_set`

## Consumes
- `l6.3-snapshot-plane`
- `l6.4-index-plane`
- `l6.5-derived-plane`
- `l6.6-artifact-plane`

## Emits
- cold planning/meta records
- reports or routing decisions
- bounded orchestration summaries

## Never owns
- hot mutation ownership
- editor widget state
- undeclared lower-layer truth
