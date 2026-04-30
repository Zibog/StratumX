# Photoreal Rendering First Lane And Visual Truth Roadmap Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze the first visualization lane for StratumX: build photoreal-capable rendering foundations before coupling advanced material physics into the viewport.

This document prevents the renderer from becoming a feature pile. The first goal is a lawful, captured, diagnosable, backend-portable visual chain that can later carry terrain deformation, material damage, hydrology, fire, smoke, wounds, fur, and living systems.

## First visualization product
The first visualization product is not a demo scene. It is the **photoreal foundation lane**:

`native window -> backend policy -> graphics port -> real backend -> frame plan -> terrain tile -> sky/atmosphere -> material visual truth -> PBR seed -> shadow seed -> exposure/post -> capture/compare -> fallback proof`

## Strict sequence
| Phase | Result | Not allowed yet |
|---|---|---|
| V0 | null backend validates frame plans | no fake present |
| V1 | backend policy and stubs visible in doctor/editor | no fake backend success |
| V2 | first real backend presents clear frame | no material fork per backend |
| V3 | framegraph/pass ownership drives the frame | no hardcoded one-off present path |
| V4 | terrain + sky + camera visible | no decorative viewport mock |
| V5 | material visual identity visible | no physics coupling requirement yet |
| V6 | PBR baseline + texture residency | no photoreal claim without capture |
| V7 | lighting/shadow/exposure baseline | no shadow feature without fallback rung |
| V8 | tunnel/flashlight/muzzle flash proof slice | no transient light without diagnostics |
| V9 | old-hardware fallback profile | no ultra claim without numeric budget |
| V10 | golden frame corpus | no release claim without retained evidence |

## Photoreal baseline definition
The first photoreal baseline requires:
- physically plausible material parameter set;
- sRGB/linear color discipline;
- camera exposure and tonemapping law;
- PBR shading family baseline;
- directional light and local light support;
- shadow map or lawful fallback shadow path;
- depth buffer and camera navigation;
- sky/atmosphere contribution;
- texture residency and missing-texture diagnostics;
- screenshot/capture with backend metadata;
- old-hardware feature rung publication.

It does not require on day one:
- full global illumination;
- full volumetric cloud simulation;
- ray tracing;
- mesh shaders;
- physics-material feedback;
- destructible material state mutation;
- character gore or fur shading.

## Visual truth before physical consequence
Material visual truth starts before material physics.

Minimum material visual truth:
- material id;
- albedo/base color;
- roughness/metalness/specular family;
- normal map slot;
- opacity/cutout posture;
- wetness visual channel placeholder;
- burn/char visual channel placeholder;
- damage/reveal visual channel placeholder;
- missing slot diagnostics.

Physics may later write wetness/burn/damage values. The renderer must already have lawful visual channels ready.

## Terrain/sky first scene
The first scene must contain:
- one world identity;
- one camera;
- one terrain tile or terrain proxy;
- one sky/atmosphere state;
- one sun/directional light;
- one material binding;
- one diagnostics rail;
- one capture artifact.

It may be visually simple. It may not bypass Graphics Port or fake the world chain.

## Tunnel proof seed
The first photoreal proof pack after terrain+sky is the tunnel/flashlight/muzzle flash seed.

Required signals:
- dark enclosed test space;
- local light cone or point light;
- transient muzzle flash emitter;
- dynamic shadow or explicit disabled-shadow reason;
- exposure response;
- capture before/after transient emission;
- old-floor fallback rung.

This seed is the first proof that StratumX can become a photoreal game engine rather than only a world simulator.

## Old-hardware law
The renderer must publish old-floor truth from the start.

Mandatory fields:
- active feature tier;
- active shadow tier;
- active texture residency tier;
- active atmosphere/media rung;
- active post-processing rung;
- disabled optional features;
- first visual compromise;
- frame budget result.

No document or UI may claim “ultra on old hardware” without a captured budget row and a visible fallback/degrade profile.

## Capture law
Every milestone from V2 onward requires a retained artifact:
- frame image or no-present proof;
- backend caps dump;
- shader target set;
- feature tier;
- disabled fast paths;
- first blocker if any;
- timestamp and build identity;
- comparison baseline when applicable.

## Current posture
`document_gold / photoreal_first_lane_defined / runtime_implementation_open`


---
# V32 Showable Frame Roadmap

## Showable Frame v1
The first public-looking StratumX frame must include:
- backend selected by policy;
- real present or lawful null/no-present outcome;
- named framegraph;
- terrain tile or proof mesh;
- sky/background contribution;
- controllable camera;
- PBR material seed;
- base color, normal, roughness/metalness/specular support or explicit fallbacks;
- directional sun contribution;
- one local/transient light slot or disabled verdict;
- shadow seed or disabled shadow verdict;
- exposure;
- tonemap;
- output color metadata;
- capture artifact and metadata;
- black-frame triage.

## Visual channels reserved before physics
The renderer must reserve but may initially neutralize:
- `wetness_visual`
- `char_burn_visual`
- `damage_reveal_visual`
- `terrain_aftermath_visual`
- `surface_breakage_visual`
- `field_debug_visual`

Material physics later writes into these channels. Renderer implementation must not wait for physics to create the channels.

## Do-not-build-yet list
The first showable image must not be blocked by:
- full destruction;
- physical wetness/hydrology;
- full volumetric weather;
- ray tracing;
- GI;
- fur;
- gore;
- NPC systems;
- dynamic fire simulation.

These become later producers of already-existing visual channels.

---

# V33 showable visual baseline addendum

Stack version: `SX-CANON/1.0.28/STACK-v34`

## Showable visual baseline

The first beautiful StratumX frame must include:

| Element | Requirement |
|---|---|
| Terrain | At least one real terrain tile from world truth, not a debug quad. |
| Sky | Sky/atmosphere baseline with sun direction and exposure contribution. |
| Material | PBR baseline with base color, roughness, normal support, missing fallback, and material id overlay. |
| Light | Directional sun, optional local light seed, shadow seed or honest fallback. |
| Camera | Valid view/projection, exposure, tone mapping, output color transform. |
| Capture | Image artifact plus backend, shader, material, framegraph, and feature tier metadata. |

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
