# Graphics Port Core Frame Plan And Backend Driver Contract Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define the engine-owned core contract between render truth and native backend drivers.

## Ownership
`graphics-port-core` owns:
- backend-neutral handles;
- backend class/status/capability types;
- frame plan schema;
- resource descriptors;
- pipeline descriptors;
- render pass nodes;
- prepared-frame contract;
- frame outcome contract;
- graphics error codes;
- no-native-type boundary tests.

It does not own:
- Vulkan/DX12/Metal object lifetimes;
- platform window creation;
- editor layout;
- material authoring UI;
- shader compiler executable ownership.

## Required core modules
- `backend_class`
- `backend_status`
- `backend_caps`
- `backend_policy`
- `feature_tier`
- `handles`
- `formats`
- `resources`
- `surface`
- `swapchain`
- `shader`
- `pipeline`
- `binding_layout`
- `frame_plan`
- `pass_graph`
- `prepared_frame`
- `frame_outcome`
- `capture`
- `diagnostics`
- `errors`

## Frame plan law
`RenderFramePlan` is the unit passed from engine render systems to a backend driver.

Required fields:
- frame id;
- world/view identity;
- camera block;
- viewport extent;
- target surface id or no-present target;
- resource use list;
- pass node list;
- material batch list;
- lighting/media requests;
- overlay requests;
- capture request;
- diagnostics request;
- feature tier request.

## Backend driver contract
A backend driver must implement coarse operations:
- `probe_caps`;
- `create_surface`;
- `create_device` or `open_device`;
- `prepare_frame`;
- `submit_prepared_frame`;
- `recover`;
- `shutdown`.

Per-draw backend calls from engine-facing code are forbidden.

## Prepared frame law
A backend may compile/lower a frame plan into `PreparedFrame`.
The prepared frame is backend-owned but frontend-visible as an opaque handle/result.

The prepared frame must publish:
- native pass count;
- resource transition count;
- pipeline cache hits/misses;
- upload bytes;
- descriptor/binding pressure;
- predicted blocker if submit is unsafe.

## Error law
All graphics-port errors must include:
- family;
- stage;
- backend class;
- first blocker code;
- recoverability;
- operator-facing summary;
- technical detail for diagnostics.

## Boundary test law
Quality tests must fail if native API types appear in non-backend crates.

## Current posture
`document_gold / core_contract_defined / implementation_open`


---
# V32 Core Contract Completion

## Required core modules
The implementation must provide role-separated modules for:
- backend class/status/caps;
- backend registry;
- backend policy resolver;
- feature tiers;
- surface/device/resource/pipeline/frame seams;
- frame plan;
- framegraph;
- prepared frame;
- present/capture outcomes;
- render failure codes;
- diagnostics.

## RenderFramePlan required fields
`RenderFramePlan` must contain:
- `frame_id`;
- `viewport_id`;
- `surface_id`;
- `camera_block`;
- `output_color_request`;
- `requested_feature_tier`;
- `visible_terrain`;
- `visible_mesh_batches`;
- `material_batches`;
- `sky_request`;
- `light_list`;
- `render_pass_nodes`;
- `resource_use_list`;
- `shader_variant_requirements`;
- `debug_overlay_intents`;
- `capture_intent`;
- `fallback_policy`.

## Failure code families
The core must define these families before backend implementation:
- `backend.*`
- `surface.*`
- `present.*`
- `frame.plan.*`
- `framegraph.*`
- `resource.*`
- `shader.*`
- `pipeline.*`
- `material.*`
- `camera.*`
- `capture.*`
- `diagnostic.*`

No backend may invent editor-facing string errors outside this vocabulary.
