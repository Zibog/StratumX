# Structural Topology Support Graph And Collapse Propagation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own exact structural-graph truth for destruction-capable world objects and buildings.

## Exact truth objects
- `StructuralNodeGraph`
- `SupportGroupSet`
- `LoadPathLedger`
- `BreachClassLedger`
- `CollapseEligibilitySet`
- `ReleasedFragmentIntentSet`

## Canonical destroyable classes
- brittle wall
- layered wall
- beam
- slab
- frame
- facade
- roof
- terrain edge

## Breach classes
| Class | Meaning |
|---|---|
| visual breach | visuals open but collision/traversal/support may remain |
| collision breach | collision envelope opens or changes |
| traversal breach | agents/projectiles may now pass or route differently |
| structural breach | support/load topology changed and may propagate collapse |

## Propagation law
Collapse propagation is legal only when support groups and load paths prove it.
Decorative fracture may not masquerade as structural collapse.

## Phase order
`damage intake -> topology mutation -> support recompute -> collapse eligibility -> fragment release -> aftermath publication`

## Failure families
- `destruction.topology_node_missing`
- `destruction.support_group_illegal`
- `destruction.collapse_propagation_forbidden`
- `destruction.breach_class_desync`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
