# Frame on Screen Delivery Chain Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

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


## v31 Graphics Port insertion
The frame-on-screen chain now explicitly includes StratumX Native Graphics Port:

`world/render truth -> frame plan -> graphics port core -> selected backend driver -> native API/platform presentation -> viewport shell -> capture/evidence`

No backend driver may be skipped by editor-only rendering. No native backend may bypass frame truth publication.


---
# V32 Image Output Closure: Exact Showable Frame Chain

## Highest law
The image-output chain is complete only when StratumX can move from world/render truth to a retained showable frame artifact without hidden renderer-specific shortcuts.

Mandatory chain:

`World truth -> render extraction -> visible set -> RenderFramePlan -> framegraph -> resource residency -> shader/pipeline resolution -> prepared backend frame -> native submit -> present/capture -> SDK packets -> tooling evidence -> editor diagnostics`

No editor viewport, runtime app, preview path, benchmark path, capture path, or first-frame path may bypass this chain.

## Exact frame lifecycle
| Step | Owner | Required input | Required output | Failure family | Editor recovery |
|---|---|---|---|---|---|
| 01 frame scope | engine runtime | world id, viewport id, clock | `FrameId`, `FrameScope` | `frame.scope.invalid` | reopen world / reset viewport |
| 02 surface state | graphics port surface seam | viewport/window/headless request | `SurfaceState` | `surface.missing` | create/rebind surface |
| 03 acquire/no-present | backend presentation seam | surface state, swapchain state | acquired image or no-present token | `present.acquire_failed` | recover swapchain / switch null |
| 04 render extraction | engine render domain | world truth, camera, render policy | render snapshot | `render.extract_failed` | inspect world/render truth |
| 05 visible set | engine render domain | snapshot, camera, culling policy | visible terrain/meshes/lights/sky | `render.visible_set_empty` | reset camera / disable culling overlay |
| 06 frame plan | graphics port core | visible set, feature tier, capture intent | `RenderFramePlan` | `frame.plan.invalid` | open frame-plan diagnostics |
| 07 residency | resource seam | frame plan resources | resident/fallback resource tickets | `resource.missing_or_unresident` | open residency lab |
| 08 shader variants | shader pipeline | material/lighting/pass requirements | shader variant keys and artifacts | `shader.variant_failed` | open shader diagnostics |
| 09 pipeline layout | pipeline seam | shader reflection, bindings, formats | pipeline keys/layouts | `pipeline.layout_mismatch` | inspect pipeline layout |
| 10 framegraph | framegraph seam | pass list, resources, attachments | ordered passes and transitions | `framegraph.invalid` | open framegraph lab |
| 11 backend prepare | backend driver | framegraph, resources, pipelines | `PreparedFrame` | `backend.prepare_failed` | backend doctor |
| 12 submit | backend driver | prepared frame, queues | submit ticket | `backend.submit_failed` | backend doctor / recover backend |
| 13 present/capture | backend presentation/capture | submit ticket, target image | `PresentOutcome`, `CaptureOutcome` | `present.failed` / `capture.failed` | recover swapchain / metadata capture |
| 14 publish | SDK/tooling/editor | frame outcome | packets, evidence, overlays | `frame.publication_failed` | save diagnostics bundle |

## Swapchain and present lifecycle
Every real on-screen backend must implement these states:
- `surface.created`
- `surface.ready`
- `surface.resize_pending`
- `surface.recreate_required`
- `surface.recreating_swapchain`
- `surface.lost`
- `surface.failed`
- `surface.no_present_headless`

Every present attempt must publish exactly one outcome:
- `presented`
- `skipped_resize`
- `swapchain_recreated`
- `surface_lost`
- `no_present_headless`
- `present_failed`
- `backend_failed_before_present`

## Frame-in-flight law
The implementation must declare frame-in-flight count and expose frame-slot state:
- `slot.free`
- `slot.cpu_recording`
- `slot.gpu_submitted`
- `slot.present_pending`
- `slot.presented`
- `slot.failed`

The editor may not mark a frame as displayed merely because CPU command recording succeeded.

## Minimum showable framegraph
A showable frame must include named nodes. A node may be a legal no-op fallback, but it must still exist in diagnostics.

| Node | Required tier | Required data | Fallback |
|---|---|---|---|
| `clear_or_load` | T1 | output target | debug clear |
| `sky_or_background` | T2 | camera, sky params | sky gradient |
| `terrain_base` | T2 | terrain/proof mesh, material | proof plane/cube only in debug |
| `opaque_material` | T3 | material batches | missing-material shader |
| `shadow_seed` | T4 | light/casters | disabled shadow verdict |
| `local_or_transient_light` | T4 | light list | capped/disabled verdict |
| `post_exposure_tonemap` | T3 | linear scene color | minimal LDR tonemap |
| `debug_overlay` | T2 | diagnostics | optional hide for final capture |
| `capture_resolve` | T1 | output color | metadata-only capture when image unavailable |
| `present_or_no_present` | T1/T0 | surface/headless target | no-present outcome |

## Output color contract
A showable frame without output color metadata is incomplete. Required metadata:
- working color space;
- HDR/LDR target;
- exposure value/source;
- tone mapper id;
- output transfer/color space;
- debug/UI composite order;
- capture color space;
- fallback if HDR/post unavailable.

## Black-frame classification
A black frame must never be silent. It must classify into exactly one primary reason:
- `black.intentional_debug_clear`
- `black.no_surface`
- `black.acquire_failed`
- `black.no_visible_set`
- `black.all_objects_culled`
- `black.no_pass_submitted`
- `black.shader_compile_failed`
- `black.pipeline_layout_mismatch`
- `black.resource_missing`
- `black.material_missing`
- `black.camera_invalid`
- `black.exposure_or_post_failed`
- `black.present_failed`
- `black.capture_failed`

Each classification must include a blocker code, recovery hint, and evidence eligibility.

## Showable frame artifact contract
The first beautiful frame must save:
- image artifact when available;
- backend class/status/caps;
- backend policy resolution;
- surface/present state;
- framegraph node list;
- shader variant list;
- material set summary;
- resource residency summary;
- output color metadata;
- fallback/degradation rungs;
- black-frame classification if relevant;
- timing summary when available.

## Acceptance
A coding agent may start graphics implementation when this document plus `121-123`, `engine/140-148`, `sdk/89-91`, `tooling/93-95`, and `editor/115/139-142` are present. It must not create a new graphics doctrine document to answer image-output questions.

---

# V33 showable frame practical completion

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Showable frame lifecycle

A showable frame must pass through this exact chain:

1. resolve backend policy;
2. create or reuse surface;
3. create or recover swapchain/present path;
4. begin frame and acquire presentation target;
5. collect render world and visible set;
6. build frame plan;
7. resolve resource residency and upload jobs;
8. resolve shader variants and pipeline keys;
9. build framegraph with declared pass dependencies;
10. record backend command packet;
11. submit and present or capture;
12. publish frame outcome, timings, blockers, and retained evidence.

Skipping any stage is allowed only in null/headless mode, and the resulting packet must say that it was null/headless rather than rendered.

---

# V34 graphics decision reinforcement

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Binding answer

StratumX Native Graphics Port remains the only graphics contract. Vulkan is the first real backend but not default engine truth. DX12, Metal, platform-native, and null are first-class backend families.

## 1.0 showable graphics target

The first beautiful frame must include:

- selected backend policy;
- real surface/swapchain/present or null headless result;
- terrain tile;
- sky;
- camera;
- one hybrid material profile;
- sun light;
- depth;
- exposure/tonemap;
- capture artifact;
- backend diagnostics.

## Not required in first frame

- final GI;
- full volumetric clouds;
- full material physics;
- destruction runtime;
- fur/hair runtime;
- full weather simulation.

Only channels and diagnostics for these future systems are required.
