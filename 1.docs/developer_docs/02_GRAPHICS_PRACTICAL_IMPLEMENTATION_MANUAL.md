# Graphics Practical Implementation Manual

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **developer guide**.


## Goal

Implement the showable graphics path through StratumX Native Graphics Port without making Vulkan the architectural center.

## Implementation order

1. implement backend registry and policy resolver;
2. implement null backend;
3. implement honest DX12/Metal/platform stubs;
4. implement Vulkan backend probe and clear frame;
5. implement surface/swapchain lifecycle;
6. implement frame-in-flight context;
7. implement framegraph pass declarations;
8. implement shader cook and variant cache;
9. implement terrain+sky baseline;
10. implement PBR material baseline;
11. implement shadow seed, exposure, tonemap;
12. implement capture metadata;
13. implement black-frame recovery.

## Completion test

The editor opens a world and displays terrain, sky, lighting, material, exposure, and capture metadata through the selected backend. `doctor` explains backend status and all failures.

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
