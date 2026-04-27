# Surface Shading Family Visual Truth And Streaming Policy Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own visual truth for hundreds of terrain and object materials at scale.

## Required family groups

| Group | Required law |
|---|---|
| terrain material sets and layer schemas | exact layer count, blend posture, cross-section policy |
| terrain cut and crater surfaces | exposed sub-layer law and aftermath variants |
| material LOD families | family-level fallback and cheapness publication |
| virtual texture / atlas / streaming policy | residency class, sampler policy, and downgrade publication |
| wet / char / damage / aftermath variants | state-driven variant rendering, not author-only overrides |
| canonical shading families | rock, soil, asphalt, metal, concrete, glass, cloth, tissue, foliage, fur, anomaly |

## Streaming policy
A surface family must declare:
- required texture families;
- lowest legal rung;
- fallback visual signature;
- residency failure verdict;
- cross-section or cut-surface behavior if the substrate is mutable.

## Failure families
- `shading.family_row_missing`
- `shading.layer_schema_illegal`
- `shading.streaming_policy_gap`
- `shading.variant_render_desync`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
