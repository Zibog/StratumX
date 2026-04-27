# Destruction Topology Mask And Collapse Authoring Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own production authoring for structural topology, support groups, fracture masks, and collapse legality.

## Mandatory tabs
- topology graph
- support groups
- fracture masks
- breach classes
- collapse propagation
- fragment release and aftermath

## Mandatory controls
- inspect topology graph;
- assign structural class;
- bind support groups and load paths;
- bind fracture mask and cross-section class;
- preview visual breach vs traversal breach vs structural breach;
- validate collapse propagation;
- capture collapse proof artifact.

## Required overlays
- support graph overlay
- load path overlay
- breach class overlay
- collapse propagation overlay
- fragment release overlay

## Required inspector fields
- `topology_ref`
- `support_group_ids`
- `load_path_ref`
- `breach_class`
- `collapse_eligibility_code`
- `fragment_release_policy_ref`
- `topology_mutation_tx_ref`

## Disabled reasons
`DST_DISABLED_NO_TOPOLOGY_BIND`, `DST_DISABLED_SUPPORT_GROUP_CONFLICT`, `DST_DISABLED_MASK_SCHEMA_INVALID`, `DST_DISABLED_COLLAPSE_PROOF_LOCKED`
