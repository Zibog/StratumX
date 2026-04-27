# Frame on Screen Delivery Chain Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the orchestration-grade chain from world truth to the visible frame on the output device.
This is not a graphics summary. It is the root delivery law for why a frame looks the way it does, which owner is responsible at each boundary, and how degraded posture and failures must be published.

## Canonical chain
`world truth -> view selection -> visibility/culling/LOD -> surface/material/shading truth -> residency/streaming readiness -> lighting/atmosphere/media -> camera/exposure/post -> composite/UI -> presentation -> device output`

## Exact handoff boundaries
| Stage | Primary owner | Mandatory inputs | Mandatory outputs | First-class failure posture |
|---|---|---|---|---|
| world truth and view selection | engine `55`, world family | world package state, placements, sky binding, active camera/view set | view scope, visible candidates, reason trace root | deny if world scope, camera profile, or view target is illegal |
| visibility, occlusion, and LOD | engine `87` | candidate set, partition state, streaming posture, traversal/occlusion state | visible set, rejected set, LOD decisions, cull reasons | every hidden object class must be attributable to one visibility decision |
| surface/material/shading truth | engine `88`, root `84–98` | surface families, material archetypes, instance stacks, response modifiers, damage-field digests | shading inputs, overlay state, material reason chain | preview may not fabricate material truth disconnected from world/material law |
| residency and resource readiness | engine `89`, `102`, `103` | textures, geometry, samplers, shader variants, warmup posture, backend feature posture | residency verdict, missing/evicted assets, active degrade rung, backend substitute verdict | low-memory posture must publish which rung changed, what was substituted, and whether a backend feature fell back |
| lighting, atmosphere, sky, media | engine `90`, `91`, `56`, root `44` | lighting rigs, probes, weather, clouds, fog, transient emission | lit scene contribution, media contribution, environment reason trace | atmosphere/lighting shortcuts may not bypass world sky bindings |
| camera, exposure, tonemap, post | engine `92` | sensor profile, exposure history, post pipeline, user view profile | presentation-ready scene buffer, exposure lineage, post reason trace | operator must be able to answer why the frame is dark, blown out, noisy, or blurred |
| composite and UI | engine `94`, `95` | scene result, VFX passes, UI/HUD state, overlay requests | final composed frame, overlay state, focus-safe diagnostics | UI or overlay composition may not hide the render blocker lineage |
| platform presentation and device output | runtime platform chain + engine `103` | final frame, swapchain/device profile, display mode, backend class, feature-tier verdict | on-screen present verdict, present timing, display fallback verdict | black screen, stale frame, or mode failure must resolve to one explicit boundary |

## Why-the-frame-looks-like-this law
A lawful frame explanation must be reconstructable from root without opening every engine file manually.
The explanation chain must include:
- which world truth scope was selected;
- which objects were culled, downgraded, or substituted;
- which material/surface rules contributed to the visible result;
- which residency rung was active;
- which backend class and optional feature posture were active;
- which lighting/atmosphere bindings were active;
- which exposure/post decisions shaped the frame;
- which overlays or UI layers were composed last.

## Material-first overlay law
Graphics is not exempt from material-first truth.
The frame chain must be able to visualize, without hidden local vocabularies:
- material family overlay;
- response profile overlay;
- thickness and cross-section family lineage when damage reveal is visible;
- terrain layer weights;
- biome/aftermath overlays;
- wetness/burn/thermal modifiers;
- damage-field / support-topology overlays when the owning route requests them;
- consequence tier visibility and sleep/wake state.

## Old-hardware proof law
A graphics route is not root-closed until the same chain explains old-floor posture.
For old-floor proof the chain must retain:
- active degrade rung;
- active backend class and optional-feature verdict;
- missing/evicted resource evidence;
- substitutions or reduced passes that became active;
- whether the result is still valid for authoring, simulation, compare, or release.

## Freeze and compare obligations
Every certification-relevant frame route must retain:
- baseline frame pointer;
- failed-run frame pointer;
- recovered-run frame pointer;
- reason trace for the first blocker or first degrade rung change;
- compare digest that proves current vs baseline vs recovered state.

## Prohibitions
- No frame law may collapse visibility, shading, and residency into one opaque “rendering” blob.
- No presentation stage may swallow the reason trace for a black/blank/wrong frame.
- No preview or debug overlay may become hidden authority for material truth.
- No backend-specific fast path may become the only legal source of material reveal.
- No old-floor path may claim success without the active degrade rung, backend posture, and retained artifacts.

## Required companion docs
- `20_GRAPHICS_CAPABILITY_MATRIX.md`
- `40_TECHNOLOGY_ORCHESTRA_CANON.md`
- `44_CROSS_DOMAIN_BUDGET_ORCHESTRA_CANON.md`
- `55_GRAPHICS_FRAME_AND_VIEW_EXTRACTION_CANON.md`
- `56_MATERIAL_LIGHTING_ATMOSPHERE_INTERACTION_CANON.md`
- `86–92` engine graphics docs
- `96–98` root material runtime docs
- `engine/103_RENDER_BACKEND_CLASS_AND_FEATURE_LADDER_CANON.md`
- `90–98` editor graphics/presentation labs

## Native backend bridge insertion law
The canonical chain includes one explicit native backend bridge between engine-facing presentation truth and platform-native presentation APIs.

Expanded tail:
`... -> composite/UI -> presentation package -> native backend bridge -> platform-present path -> device output`

The native backend bridge:
- may lower to Direct3D 12, Vulkan, Metal, or a platform-native port adapter;
- may publish native escape hatches only through explicit capability verdicts;
- may not become a second truth owner for frame, material, or capture law.

## Honest viewport law
The first visible editor viewport must consume the same lawful frame chain as runtime presentation.
A decorative placeholder viewport, mock background, or fake preview card is forbidden as the primary world-editing surface.
If the world fails to open, the viewport must stay present and explain the failure in-place rather than switching to unrelated splash chrome.

## Split-view law
Secondary orthographic or detached viewports are legal.
They are subordinate shells over the same frame truth and must publish:
- active view class;
- shared camera or independent camera posture;
- capture eligibility;
- reduced-quality or throttled-update posture when the shell is under pressure.
