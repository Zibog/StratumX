# Native Backend Bridge Port And Platform Adapter Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the implementation law for native backend modules and future platform-port adapters.

## Bridge law
The bridge sits between engine render truth and native backend modules.
It owns:
- canonical device/queue identities;
- resource-lifetime publication;
- pass and synchronization lowering;
- present-path lowering;
- diagnostics and capture verdict publication.

## Required module families
- `rhi.backend.dx12`
- `rhi.backend.vulkan`
- `rhi.backend.metal`
- `rhi.backend.null`
- `rhi.backend.platform_port.*`

## Platform-port adapter law
A platform-port adapter may:
- wrap platform-native memory/resource lifetimes;
- own platform-specific presentation details;
- expose explicit optional accelerators.

It may not:
- invent second material truth;
- bypass frame-trace publication;
- bypass capture/certification legality;
- hide backend class or shader-target posture.

## Publication requirements
Every active backend module or adapter must publish:
- backend class id;
- shader target set;
- feature-tier verdicts;
- optional-accelerator verdicts;
- first disabled fast path;
- capture legality verdict;
- present verdict or no-present verdict.

## Companion docs
- engine `103`
- engine `104`
- root `100`
