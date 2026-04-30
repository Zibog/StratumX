# Graphics Port DX12 Metal And Platform Stub Backend Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define required stub backend drivers for Direct3D 12, Metal, and future platform-native graphics APIs so StratumX keeps first-class backend slots before full implementation.

## Stub law
A stub backend is a truthful reserved implementation slot.

It must:
- compile when its feature/platform allows;
- register a backend descriptor;
- publish intended backend class;
- publish shader target set;
- publish implementation status `stubbed`;
- publish first blocker;
- participate in doctor and editor backend lists;
- refuse present/capture with explicit failure.

It must not:
- claim availability;
- fake a frame outcome;
- panic instead of returning blocker;
- leak native API types above backend boundary.

## Required stubs
| Stub | Backend class | First blocker |
|---|---|---|
| DX12 | `graphics.backend.dx12` | `backend.dx12.device_bootstrap_missing` |
| Metal | `graphics.backend.metal` | `backend.metal.device_bootstrap_missing` |
| Platform native | `graphics.backend.platform_native.*` | `backend.platform_native.sdk_not_configured` |

## Upgrade path
A stub becomes a real backend only after:
- device/surface boot works;
- caps publication works;
- frame plan preparation works;
- present/no-present law works;
- capture metadata works;
- quality tests prove no frontend changes were required.

## Current posture
`document_gold / stubs_required / implementation_open`


---
# V32 Stub Backend Completion

## Required stubs
- `graphics.backend.dx12.stub`
- `graphics.backend.metal.stub`
- `graphics.backend.platform_native.stub`

## Stub behavior
Each stub must compile or report `not_built`, publish backend class, publish shader target intent, publish status, publish first blocker, reject render submission, appear in render doctor, and pass no-fake-success tests.

## Real backend migration law
When a stub becomes real, keep backend class id and SDK packet shape. Add device/surface/resource/pipeline/frame seam implementation. Do not change material truth, editor controls, tooling route names, or showable-frame acceptance criteria.
