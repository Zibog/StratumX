# StratumX Native Graphics Port And Backend Driver Constitution Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze the StratumX-owned graphics portability layer that lets the engine, SDK, tooling, editor, and future games talk to one render contract while native backend drivers lower that contract to Vulkan, Direct3D 12, Metal, platform-native console APIs, or null/headless validation.

This document replaces any interpretation that Vulkan is the architectural center of the renderer. Vulkan is the first real backend implementation candidate, not the default renderer and not the owner of StratumX graphics truth.

## Canonical name
The canonical term is **StratumX Native Graphics Port**.

Allowed short forms:
- `Graphics Port Layer`
- `Native Graphics Port`
- `SX graphics port`

Forbidden active-canon names:
- `Render Wire`
- `RenderWare clone`
- `Vulkan renderer`
- `universal graphics API`
- `generic renderer`

The historic inspiration is the production philosophy of a portable game-rendering stack. The name and ownership are StratumX-only.

## Highest law
No native graphics API is canonical truth.

The canonical chain is:

`world/render truth -> visible set -> render frame plan -> StratumX Native Graphics Port -> selected backend driver -> native API/platform presentation`

The following may never depend on Vulkan, Direct3D 12, Metal, or console SDK types:
- material truth;
- world visibility truth;
- terrain rendering intent;
- sky/weather rendering intent;
- lighting intent;
- editor viewport law;
- SDK packet law;
- tooling capture/recovery law;
- certification/golden corpus law.

Only backend-driver crates may import native API bindings or platform SDK graphics types.

## Layer stack
| Layer | Owns | Must not own |
|---|---|---|
| Engine render domain | visible sets, material render intent, light intent, sky/media intent, camera view, render feature requests | native API objects, swapchain internals, backend-specific descriptors |
| Graphics Port core | frame plan, resource descriptors, backend registry, capability publication, policy resolver, diagnostic/error vocabulary | backend-specific memory allocation, native command buffer syntax |
| Backend drivers | native device/surface/swapchain, command recording, resource lowering, pipeline lowering, native sync, present/capture hooks | engine material truth, editor UI truth, gameplay/workflow decisions |
| SDK | backend capability packets, frame outcome packets, render failure packets, capture packets | native API handles |
| Tooling | backend probe routes, render doctor, shader cook, frame capture/compare/recovery | backend truth, engine render truth |
| Editor | operator-visible backend badges, viewport policy selection, diagnostics, capture/recovery controls | backend-specific implementation |

## Required backend families
| Backend family | Class id | Initial posture | Runtime role |
|---|---|---|---|
| Null/headless | `graphics.backend.null` | mandatory immediately | CI, validation, no-present proof, tests |
| Vulkan | `graphics.backend.vulkan` | first real native backend | Linux/Windows portable path, locked benchmark path, explicit GPU model proof |
| Direct3D 12 | `graphics.backend.dx12` | mandatory stub immediately, real backend later | preferred Windows production path when implemented |
| Metal | `graphics.backend.metal` | mandatory stub immediately, real backend later | preferred Apple production path when implemented |
| Platform-native | `graphics.backend.platform_native.*` | mandatory reserved stub immediately | console/restricted platform ports after legal SDK access |

## Backend policy law
Backends are selected by **policy**, not by hard-coded renderer identity.

Required policies:
- `policy.auto`
- `policy.forced_backend`
- `policy.benchmark_locked`
- `policy.headless_validation`
- `policy.editor_preview`
- `policy.capture_recovery`

Canonical platform preferences:
| Platform | Preferred policy resolution | Required fallback |
|---|---|---|
| Windows | Direct3D 12 when implemented, Vulkan as portable fallback, null for headless | Vulkan or null |
| Linux | Vulkan, null for headless | null |
| macOS | Metal when implemented, null for headless | null |
| restricted/console | platform-native backend when legal SDK is present | null/dev-only |
| CI | null/headless | none |

Vulkan may be forced for benchmark parity. It may not be silently treated as the only renderer.

## Backend status law
Every backend publishes one status:
- `available`
- `stubbed`
- `disabled`
- `failed`
- `not_built`
- `not_legal_on_platform`
- `sdk_unavailable`

A stub backend may compile and participate in policy/doctor/capability tests. It may never claim a successful present or a successful capture.

## No fake success law
A backend stub must return a first blocker code before frame submission:
- `backend.dx12.device_bootstrap_missing`
- `backend.metal.device_bootstrap_missing`
- `backend.platform_native.sdk_not_configured`
- `backend.platform_native.platform_contract_unavailable`

The editor may show these as reserved backend slots. It may not show them as working renderers.

## Invariant vs variant law
Invariant across all backends:
- material render intent;
- render feature tiers;
- frame plan shape;
- capture metadata shape;
- editor diagnostics shape;
- fallback/degrade vocabulary;
- shader source identity;
- asset/material/texture canonicalization identity.

Variant by backend:
- native device/surface/swapchain creation;
- memory allocation implementation;
- descriptor/binding implementation;
- command recording syntax;
- synchronization/barrier lowering;
- pipeline binary format;
- platform presentation details;
- backend profiler/capture hook implementation.

## Five seam law
The Graphics Port Layer is split into five small seams, not one giant abstraction:

1. **Surface seam** — window/headless/editor viewport/platform surfaces.
2. **Device seam** — adapter selection, device creation, queues, capabilities.
3. **Resource seam** — buffers, images, samplers, uploads, residency tickets.
4. **Pipeline seam** — shader variants, pipeline keys, binding layouts, material pipeline identities.
5. **Frame seam** — frame plans, prepared frames, command packets, submit, present, capture.

A change in one seam must not force a rewrite of the other four seams.

## Performance law
The frontend may use traits or enums for coarse backend control. The hot path may not dispatch one virtual call per draw command.

Required performance rules:
- frame work is batched into `RenderFramePlan` / `PreparedFrame` / backend command packets;
- handles are stable typed ids, not native handles;
- backend internals may use backend-specific arenas/caches/pools;
- render passes are compiled or prepared before submit where possible;
- optional fast paths are exposed as capability verdicts;
- no backend-specific fast path may become the only correct rendering path.

## Raw native type containment
Forbidden outside backend-driver crates:
- `vk::*`, `Vk*`, `ash::vk::*`;
- `ID3D12*`;
- `MTL*` / Metal object types;
- console SDK graphics object types.

Allowed outside backend-driver crates:
- `BackendClass`;
- `BackendCaps`;
- `BackendStatus`;
- `RenderFramePlan`;
- `RenderFeatureTier`;
- `PresentResult`;
- `CaptureResult`;
- `GraphicsPortErrorCode`.

## Photoreal-first posture
The first production target for this layer is photoreal rendering foundation, not physics-material coupling.

Order:
1. honest backend selection;
2. null + stubs;
3. Vulkan real backend pulse;
4. first honest viewport;
5. terrain + sky;
6. material visual truth;
7. PBR + shadows + exposure;
8. tunnel/flashlight/muzzle flash proof;
9. old-hardware fallback rungs;
10. capture/compare/golden evidence.

Material physics may couple later. Material **visual truth** must be present from the start.

## Required companion docs
- root `20`, `42`, `100`, `102`, `114`, `122`, `123`
- engine `103`, `104`, `107`, `140–148`
- sdk `89–91`
- tooling `93–95`
- editor `139–142`

## Current posture
`document_gold / graphics_port_authoritative / implementation_open`


---
# V32 Execution-Grade Addendum: Backend Driver Is Not Renderer Truth

## Implementation package roles
The implementation must have these roles either as crates or as clearly separated modules in existing crates:
- `graphics_port_core`
- `graphics_port_backend_null`
- `graphics_port_backend_vulkan`
- `graphics_port_backend_dx12_stub`
- `graphics_port_backend_metal_stub`
- `graphics_port_backend_platform_stub`
- `graphics_port_shader_pipeline`
- `graphics_port_framegraph`
- `graphics_port_resource_residency`
- `graphics_port_capture_diagnostics`

Do not create duplicate packages if an existing package already owns the role. Rename only when it improves clarity.

## Stable public types
The Graphics Port public surface must expose typed StratumX ids, not native handles:
- `BackendClass`
- `BackendStatus`
- `BackendCaps`
- `BackendPolicy`
- `BackendResolution`
- `FeatureTier`
- `OptionalFeatureVerdict`
- `SurfaceId`
- `FrameId`
- `ResourceHandle`
- `ShaderVariantKey`
- `PipelineKey`
- `RenderFramePlan`
- `PreparedFrame`
- `FrameOutcome`
- `PresentOutcome`
- `CaptureOutcome`
- `RenderFailureCode`

## Backend interface law
Backend calls must be coarse-grained. The hot path may not dispatch through a trait object per draw call.

Canonical shape:

```rust
pub trait GraphicsBackend {
    fn caps(&self) -> &BackendCaps;
    fn create_surface(&mut self, request: SurfaceRequest) -> Result<SurfaceId, RenderFailure>;
    fn prepare_frame(&mut self, plan: &RenderFramePlan) -> Result<PreparedFrame, RenderFailure>;
    fn submit_prepared_frame(&mut self, frame: PreparedFrame) -> Result<FrameOutcome, RenderFailure>;
    fn recover(&mut self, request: RecoveryRequest) -> RecoveryOutcome;
}
```

Backends may use native caches, arenas, command pools, descriptor pools, or platform-specific fast paths internally. These must not leak into engine/editor/SDK public types.

## No rewrite guarantee
A new backend may require:
- backend driver implementation;
- backend shader target;
- backend surface adapter;
- backend capture/profiler hook;
- backend feature verdict table.

A new backend must not require rewriting:
- material visual truth;
- terrain rendering intent;
- editor viewport controls;
- framegraph node vocabulary;
- SDK packet shapes;
- tooling doctor/capture routes;
- showable-frame acceptance rules.

## Acceptance
This constitution is complete only if a coding agent can add `dx12_real` or `metal_real` later without touching material law, editor viewport law, SDK packet law, or tooling route law.

---

# V33 practical graphics closure addendum

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Practical backend driver completion

Every backend driver must implement or honestly stub the following surfaces:

| Surface | Required behavior |
|---|---|
| Probe | Detect availability, device candidates, feature tier, shader target, presentation mode, capture support. |
| Surface | Create platform surface or return exact unsupported reason. |
| Swapchain | Create, recreate, resize, recover, and report present state. |
| Frame | Accept prepared frame packet, execute or reject with exact blocker. |
| Capture | Produce capture artifact or exact unavailable code. |
| Diagnostics | Report black-frame, shader failure, present failure, resource pressure, and disabled fast paths. |

A stub backend is valid only if it participates in policy resolution and returns honest blockers. It must never fake a rendered frame.

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
