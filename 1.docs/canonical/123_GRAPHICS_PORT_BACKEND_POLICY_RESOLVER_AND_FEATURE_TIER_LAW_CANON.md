# Graphics Port Backend Policy Resolver And Feature Tier Law Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze backend policy selection and render feature tiers so StratumX can add Vulkan, Direct3D 12, Metal, and platform-native ports without rewriting engine/editor code and without lying about unsupported capabilities.

## Resolver law
Backend selection is resolved by `GraphicsBackendPolicyResolver`.

Inputs:
- requested policy;
- platform family;
- build features;
- detected backend availability;
- required presentation mode;
- required capture mode;
- benchmark lock posture;
- editor/runtime/headless context;
- user override.

Outputs:
- selected backend class;
- selected backend status;
- fallback chain;
- first blocker;
- warning list;
- feature tier;
- capture posture;
- present posture.

## Required policies
| Policy | Use | Must publish |
|---|---|---|
| `auto` | normal editor/runtime launch | selected backend and reason |
| `forced_backend` | user/dev explicit backend choice | forced backend and failure if impossible |
| `benchmark_locked` | retained comparison or performance proof | backend lock and reproducibility reason |
| `headless_validation` | CI, server, schema/frame validation | null/no-present proof |
| `editor_preview` | editor viewport with diagnostics | backend, degrade rung, present/capture readiness |
| `capture_recovery` | failed frame recovery | original failure + recovered path |

## Feature tiers
| Tier | Name | Meaning |
|---|---|---|
| `T0` | null/headless | validates frame plans and packets, no present |
| `T1` | basic present | clear frame, swapchain/present, diagnostics |
| `T2` | world viewport | camera, terrain/mesh proxy, sky, depth |
| `T3` | material viewport | material visual truth, texture slots, overlays |
| `T4` | photoreal baseline | PBR, shadows, local lights, exposure/post |
| `T5` | photoreal proof | tunnel/flashlight/transient light proof + capture |
| `T6` | advanced media | volumetric/media/GI/probe advanced paths where available |

A backend may support a higher tier partially. Partial support must publish missing feature rows and fallback rungs.

## Optional accelerator law
Optional accelerators include:
- ray tracing;
- mesh/task shaders;
- descriptor indexing/bindless;
- variable rate shading;
- async compute;
- timeline semaphores;
- dedicated transfer queues;
- sparse/tiled resources;
- vendor capture/profiling hooks.

Optional accelerators are never required for core photoreal baseline. They may improve quality/performance after the common path works.

## Fallback law
Fallback is not failure when it is lawful and visible.

Examples:
- no advanced shadows -> lower shadow tier with visible badge;
- no volumetrics -> summary media rung with horizon truth preserved;
- no bindless -> bounded descriptor table path;
- no async compute -> graphics-queue serialized path;
- no present surface -> null/no-present validation path.

## Doctor law
`tools/doctor --render-backends` must be able to answer:
- which backend classes are compiled;
- which are available on this machine;
- which are stubbed;
- which are disabled by platform/build;
- why the selected backend was chosen;
- what the fallback chain is;
- what the first blocker is.

## Editor law
Editor viewport must expose:
- policy;
- selected backend;
- status;
- feature tier;
- fallback rung;
- disabled optional features;
- capture readiness;
- present readiness;
- first blocker.

## Current posture
`document_gold / backend_policy_resolver_defined / implementation_open`


---
# V32 Resolver Completion Table

## Required resolver output
Every backend resolution returns:
- requested policy;
- selected backend class;
- selected backend status;
- feature tier;
- shader target set;
- fallback chain;
- rejected backend reasons;
- first blocker;
- present capability;
- capture capability;
- old-floor/degrade rung;
- benchmark lock state;
- headless state.

## Feature tiers
| Tier | Name | Required output |
|---|---|---|
| T0 | null/headless | validates frame plan and publishes no-present outcome |
| T1 | basic present | real surface/present or declared failure |
| T2 | terrain+sky | terrain/proof mesh, sky/background, camera |
| T3 | PBR baseline | material visual truth, output color chain |
| T4 | shadows/lights | shadow seed or disabled verdict, local/transient light slot |
| T5 | photoreal proof | capture/compare, old-floor fallback, showable proof |
| T6 | advanced media | volumetrics/probes/reflections/advanced streaming later |

## Stub completion law
DX12, Metal, and platform-native stubs are complete only when:
- they are registered;
- they publish caps shape;
- they publish first blocker;
- doctor can inspect them;
- forced selection fails honestly;
- they never return successful present/capture.

---

# V33 backend policy resolver completion addendum

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Backend policy resolver completion

Resolver outputs must include:

- requested policy;
- selected backend;
- fallback chain;
- platform reason;
- feature tier;
- shader target set;
- disabled optional features;
- first blocker;
- editor-visible explanation;
- tooling doctor verdict;
- reproducible command line equivalent.

The resolver must support `auto`, `forced`, `benchmark_locked`, `headless`, `editor_preview`, and `capture_recovery` policies.

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
