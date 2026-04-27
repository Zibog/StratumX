# Graphics Capability Matrix

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Map graphics capability families to the active production contour using one vocabulary: truth owner, owner surfaces, button namespace, canonical pack ids, the root frame delivery chain frozen in `42`, and the backend / feature ladder frozen in engine `103–104`.

## Matrix
| Capability family | Root chain role | Engine truth owners | Editor production surfaces | Button namespace | Canonical pack ids | Current posture |
|---|---|---|---|---|---|---|
| frame timing and budget | frame scope, pacing, budget visibility | `engine/86` | `editor/90` | frame command rows in `editor/110` | `pack.frame_budget_triplet` | `doc_closed_impl_open` |
| pipeline and frame graph | pass ownership and composition order | `engine/86 + engine/87 + engine/103 + engine/104` | `editor/91` | render-pipeline command rows in `editor/110` | `pack.render_pipeline_frame_graph` | `doc_closed_impl_open` |
| materials and shader/texture truth | surface/material/shading stage, material-first overlays | `engine/88` + root `96–98` + `engine/106` where coverage visuals are active | `editor/92` + `editor/113` | render-material command rows in `editor/110` | `pack.material_shader_texture + pack.material_surface_response` | `doc_closed_impl_open` |
| residency and memory pressure | residency/streaming readiness and degrade rung | `engine/89 + engine/102 + engine/103 + engine/104` | `editor/93` | residency command rows in `editor/110` | `pack.texture_residency_memory` | `doc_closed_impl_open` |
| lighting, sky, atmosphere | lighting/environment/media contribution | `engine/90 + engine/91 + engine/92 + engine/106` | `editor/94` | sky/atmosphere presentation command rows in `editor/110` | `pack.storm_long_range_visibility + pack.tunnel_flash_shadow_media` | `doc_closed_impl_open` |
| VFX composite and media pressure | transient media and composite chain | `engine/94` | `editor/97` | VFX command rows in `editor/110` | `pack.vfx_composite_media + pack.tunnel_flash_shadow_media` | `doc_closed_impl_open` |
| UI runtime composition and text | final overlay and presentation chain | `engine/95` | `editor/98` | UI command rows in `editor/110` | `pack.ui_runtime_layout_text` | `doc_closed_impl_open` |
| backend and feature posture | runtime backend choice, capability query, optional accelerators, forbidden baseline shortcuts, backend escape hatches | `engine/103 + engine/104` | `editor/101 + editor/102 + editor/103` | platform / fallback / certification rows in `editor/110` | `pack.platform_fallback_certification` | `document_gold` |
| living layered surface traversal | garment/skin/tissue/bone traversal visibility at render/runtime seam | `engine/105 + engine/71 + engine/67` | `editor/72 + editor/113` | wound / material command rows in `editor/110` | `pack.ballistics_wound_trace + pack.material_surface_response` | `doc_closed_impl_open` |
| fur and hair coverage posture | coverage masks, promoted-local strand mode, wind/wet/char state | `engine/63 + engine/106` | `editor/68 + editor/113` | soft-surface command rows in `editor/110` | `pack.fur_cloth_weather + pack.fur_cloth_weather_wind` | `doc_closed_impl_open` |

## Law
These matrices may not invent local namespaces, local pack aliases, or local route names outside root `42`, `69`, `71`, `72`, `74`, `76`, `81`, engine `103–106`, and `editor/110`.

## Backend-neutral rendering law
The rendering architecture must stay backend-neutral above the runtime backend seam.
Material truth, streaming truth, frame-graph truth, living-layer truth, and certification truth may not be rewritten around one optional GPU feature family.

## Common-denominator plus optional-accelerator law
The graphics stack must keep one common minimum path and may add faster backend-specific or hardware-specific accelerators only as optional tiers.
No optional accelerator may become the only lawful way to render materials, coverage systems, or certification captures.

## Expansion rows
| Capability family | Root chain role | Engine truth owners | Editor production surfaces | Button namespace | Canonical pack ids | Current posture |
|---|---|---|---|---|---|---|
| native backend bridge and platform ports | backend-neutral frontend, native lowering, platform-port diagnostics | `engine/103 + engine/104 + engine/107` + root `100` | `editor/101`, `editor/102`, `editor/103`, `editor/115` | platform / viewport / certification rows in `editor/110` | `pack.render_backend_bridge` | `document_gold` |
| viewport host and presentation shell | honest primary viewport, split-view posture, layout-aware capture | `engine/86–92` + `engine/103–104` | `editor/114 + editor/115` | viewport / window rows in `editor/110` | `pack.editor_viewport_shell + pack.editor_workspace_layout` | `doc_closed_impl_open` |


## Dream-stack expansion rows
| Capability family | Root chain role | Engine truth owners | Editor production surfaces | Button namespace | Canonical pack ids | Current posture |
|---|---|---|---|---|---|---|
| giant-world observability and distant representation | far visibility, storm/fire/squad/theater presentation | `engine/108 + engine/114 + engine/115` | `editor/122 + editor/126 + editor/127` | climate / cert / view rows in `editor/110` | `pack.world_geodesy_far_causality + pack.climate_front_storm_theater + pack.distant_world_representation` | `document_gold` |
| terrain visual truth at scale | terrain set binding, cross-sections, variants, layer schema | `engine/111 + engine/130` | `editor/119` | terrain/material rows in `editor/110` | `pack.terrain_deformation_persistence + pack.surface_shading_truth` | `document_gold` |
| photoreal old-hardware fallback | old-floor lighting/media/rung legality | `engine/126` | `editor/126` | certification rows in `editor/110` | `pack.photoreal_old_hardware` | `document_gold` |
| foliage/fur/hair microgeometry | representation ladder, wetness/char, silhouette law | `engine/127 + engine/106` | `editor/125` | fur/groom rows in `editor/110` | `pack.foliage_fur_microgeometry` | `document_gold` |
| VFX taxonomy and truth separation | debris/smoke/sparks/fluid/ash/dust families | `engine/129` | `editor/97 + editor/120 + editor/121 + editor/122` | VFX and destruction rows in `editor/110` | `pack.vfx_runtime_taxonomy` | `document_gold` |
