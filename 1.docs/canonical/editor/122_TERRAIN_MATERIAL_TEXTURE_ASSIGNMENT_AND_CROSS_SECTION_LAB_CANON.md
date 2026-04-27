# Terrain Material Texture Assignment And Cross Section Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Provide the dedicated authoring lane for exposed cuts, craters, and cross-section readability after deformation or destruction.

## Mandatory controls
- choose cross-section family by terrain/material set;
- preview cut wall and crater edge result;
- bind sub-layer visual families;
- inspect wet / char / damage variant behavior inside cuts;
- compare pre-cut and post-cut captures.

## Required overlays
- exposed-layer stack overlay
- cross-section texture atlas overlay
- variant-state overlay

## Required inspector fields
- `cross_section_family_ref`
- `sub_layer_visual_rows`
- `crater_edge_variant_ref`
- `cut_surface_streaming_policy`

## Disabled reasons
`XSC_DISABLED_NO_MUTABLE_SUBSTRATE`, `XSC_DISABLED_LAYER_STACK_MISSING`, `XSC_DISABLED_ATLAS_INCOMPATIBLE`
