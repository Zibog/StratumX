# Physics Authoring And Debug Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Phase-2 owner surface for material-law authoring and terrain/crater proof.
This file covers the editor-facing `Material Law Editor` and `Terrain/Crater Authoring Surface`.

## Command inventory
- `btn.material.author_response_profile`
- `btn.material.bind_surface_family`
- `btn.material.inspect_response_table`
- `btn.terrain.simulate_blast_profile`
- `btn.terrain.compare_blast_triplet`
- `btn.terrain.capture_blast_evidence`
- `btn.terrain.recover_blast_baseline`
- `btn.terrain.certify_blast_pack`

## Required overlays
- `overlay.material.response_matrix`
- `overlay.terrain.crater_profile`
- `overlay.terrain.ejecta_field`
- `overlay.terrain.surface_delta`
- `overlay.terrain.failure_board`

## Required inspector fields
- `material_response_profile_id`
- `surface_family`
- `fracture_class`
- `debris_class`
- `crater_response_class`
- `wetness_response_class`
- `penetration_response_class`
- `terrain_baseline_ref`
- `artifact_ref`

## Disabled reasons
- `disable.material.author_blocked`
- `disable.material.bind_blocked`
- `disable.material.inspect_blocked`
- `disable.terrain.simulate_blocked`
- `disable.terrain.capture_blocked`

## Exact brutal proof slices
1. grenade in mud.
2. grenade on asphalt.
3. compare both outputs without collapsing them into one surface-family result.
4. recover last-good baseline and rerun the triplet.

## Compare and capture law
This surface may use only:
- `compare.material.response_triplet`
- `capture.material.response_bundle`
- `compare.terrain.blast_triplet`
- `capture.terrain.blast_bundle`

Certification packs allowed from this surface:
- `pack.material_surface_response`
- `pack.terrain_crater_truth`
- `pack.terrain_blast_degrade`

## Registry dependencies
- root `84` freezes archetype and surface-family registry law used here;
- root `85` freezes response profile and instance stack law used here;
- root `86` freezes terrain-layer and aftermath overlay law consumed by terrain blast and crater routes;
- root `87` freezes sleep/wake and cheap consequence tier law for all simulated material responses.

## Current posture
`document_gold / doc_closed_impl_open`
