# Native Graphics Bridge And Platform Portability Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Freeze one engine-owned bridge between renderer truth and native graphics APIs so the archive can ship on Windows, Linux, macOS, and future platform ports without turning the renderer into a fake universal API religion.

## Core law
The engine owns:
- one backend-neutral frontend contract;
- one thin native bridge;
- one set of native backend modules or platform-port adapters.

The engine does **not** own:
- a giant least-common-denominator graphics API that tries to look identical everywhere;
- a hidden third-party wrapper that silently becomes the real authority;
- backend-specific truth above the bridge.

## Canonical layers
`render truth / frame graph / residency / lighting / camera -> engine-owned bridge -> native backend module or platform-port adapter -> platform presentation`

## Required backend families
- `rhi.backend.dx12` for primary Windows runtime
- `rhi.backend.vulkan` for portable parity and locked benchmark paths
- `rhi.backend.metal` for Apple-platform native runtime
- `rhi.backend.null` for headless validation and no-present proof
- `rhi.backend.platform_port.*` for future platform-native ports where the platform requires its own backend ownership model

## Platform-port law
A console or restricted platform may require one platform-native adapter family.
That adapter is legal only when:
- the bridge stays authoritative above it;
- capability publication remains explicit;
- capture, diagnostics, and failure posture stay comparable;
- material, residency, and frame law remain backend-neutral above the seam.

## Shader-target law
The canonical authoring language remains HLSL-first.
Native outputs may include:
- DXIL for Direct3D 12;
- SPIR-V for Vulkan;
- Metal-target output for Metal paths;
- platform-native compiled outputs for restricted ports.

The archive may specialize shaders per backend.
It may not fork material truth, capture truth, or certification truth per backend.

## Portability law
Cross-platform closure comes from:
- stable engine-facing contracts;
- explicit capability publication;
- explicit optional-accelerator ladders;
- explicit platform-port adapters.

It does **not** come from pretending Direct3D 12, Vulkan, Metal, and future platform APIs are identical.

## Performance law
The bridge must stay thin enough that:
- command recording ownership is obvious;
- synchronization ownership is obvious;
- resource lifetime is explicit;
- present-path diagnostics survive fallback;
- backend-specific fast paths remain optional and measurable.

## Prohibitions
- no backend-specific shader source as the only legal material path;
- no one backend may be the only legal capture path;
- no platform port may swallow diagnostics;
- no hidden wrapper may become the real graphics truth;
- no editor viewport may consume mock graphics output while runtime uses a different lawful chain.

## Required companion docs
- root `42`
- root `100`
- engine `103`, `104`, `107`
- editor `115`
- sdk `64`, `65`, `67`, `78`
- tooling `67`, `68`, `82`


## v31 clarification: StratumX Native Graphics Port
The active architecture is now named **StratumX Native Graphics Port**.
The previous wording “native graphics bridge” remains compatible as a description, but the authoritative implementation target is the Graphics Port Layer defined in root `121`.

Vulkan is a required first real backend candidate. It is not the default renderer, not the architectural center, and not the shape of engine-facing render APIs.

The bridge must be implemented as backend drivers below a StratumX-owned contract:
- null/headless mandatory immediately;
- Vulkan first real backend;
- Direct3D 12, Metal, and platform-native stubs mandatory immediately;
- backend policy resolver mandatory before editor viewport relies on any backend.
