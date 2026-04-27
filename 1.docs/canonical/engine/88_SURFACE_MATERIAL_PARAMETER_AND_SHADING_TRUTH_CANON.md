# Surface Material Parameter and Shading Truth Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own surface-level material parameters, shading families, and legal surface response inputs.

## Truth objects
MaterialTruth, SurfaceParameterBlock, ShaderFamilyRef

## Runtime phases
surface resolve -> parameter bind -> shading family select -> response publish

## Diagnostics
missing material instance, invalid parameter domain, unsupported shading family

## Exact editor entrypoints
- render-material command rows in `editor/110` via `editor/92`

## Required publications
- `obs.material.*` with material truth id, shader family, fallback verdict, and blocker code;

## Phase-4 brutal proof slices
- `material_truth_and_fallback_legality_publish_same_frame`;
- `unsupported_shader_family_emits_typed_denial`;

## Old-floor evidence obligations
- one capture bundle proving fallback legality stays explicit under pressure;
- one compare digest proving baseline/current/recovered material truth;

## Current posture
`document_gold / doc_closed_impl_open / runtime_family_closed_in_docs`
