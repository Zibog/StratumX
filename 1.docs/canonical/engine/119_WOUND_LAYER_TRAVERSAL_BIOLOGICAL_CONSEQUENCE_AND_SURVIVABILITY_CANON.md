# Wound Layer Traversal Biological Consequence And Survivability Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Separate layered material traversal from biological consequence without losing the link between them.

## Layer stack
`garment -> skin -> soft_tissue -> organ_zone -> bone -> exit / lodged`

## Exact truth objects
- `WoundTraversalLedger`
- `BodyTopologyClass`
- `GoreLegalityVerdict`
- `SurvivabilityVerdict`
- `HealingScarLedger`

## Law
- material traversal rows decide contact through layers;
- biological consequence rows decide pain, bleed, shock, survivability, severance, and long-horizon scarring;
- non-human species may remap topology classes and survivability rules without breaking the layer contract.

## Failure families
- `wound.layer_order_break`
- `wound.body_topology_missing`
- `wound.gore_legality_gap`
- `wound.survivability_verdict_missing`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
