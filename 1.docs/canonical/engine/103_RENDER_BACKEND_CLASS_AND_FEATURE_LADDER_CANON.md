# Render Backend Class And Feature Ladder Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the runtime backend seam, capability-query law, optional-accelerator ladder, and forbidden baseline shortcuts for graphics.

This document exists so the engine stops hand-waving around backend choice.
The archive must answer three different questions cleanly:
- what the architecture is allowed to depend on;
- what the locked benchmark backend is;
- which GPU features are optional accelerators rather than baseline law.

## Backend architecture law
The renderer above the backend seam is backend-neutral.
The following truths may not depend on one backend-specific feature family or one platform-port adapter:
- material truth;
- frame-graph truth;
- residency truth;
- old-floor degrade truth;
- living-layer traversal visibility truth;
- coverage-system truth for grass / fur / hair / feathering;
- capture / compare / certification truth.

## Canonical backend classes
| Backend class id | Meaning | Canonical use |
|---|---|---|
| `backend.vk_rt_locked` | locked Vulkan realtime benchmark backend | locked baseline captures and benchmark continuity |
| `backend.dx12_primary_windows` | Direct3D 12 primary Windows runtime backend | Windows-first shipping and day-to-day runtime path when selected by product profile |
| `backend.vk_portable_runtime` | Vulkan portable runtime backend | non-Windows or backend-parity runtime path when enabled |
| `backend.metal_primary_apple` | Metal primary Apple runtime backend | Apple-platform shipping and day-to-day runtime path when selected by product profile |
| `backend.platform_port_native` | restricted platform-native backend family | platform-specific port where the platform requires native ownership patterns |
| `backend.null_headless` | non-presenting validation backend | server, batch validation, or no-frame proof routes |

## Backend selection law
- locked benchmark and baseline capture routes may remain pinned to `backend.vk_rt_locked` for continuity;
- a Windows-first shipping product may legally select `backend.dx12_primary_windows` as the primary runtime path;
- optional parity work for `backend.vk_portable_runtime` may not rewrite root material or frame law;
- backend choice must be published as explicit runtime posture, not hidden inside the build.

## Thin-RHI law
The engine must own one thin runtime hardware interface above backend modules.
That seam is defined in engine `104`.
Translation layers or third-party portability wrappers may be used for experiments, tooling, or bring-up, but they may not silently replace the canonical engine-owned backend seam.

## Feature-ladder law
The engine must divide features into three bands.

### Band A — required common minimum
These are allowed to participate in the common denominator runtime:
- command queues / command lists or command buffers;
- render and compute passes;
- indirect draw / dispatch support where present in the selected backend profile;
- ordinary descriptor / binding model;
- standard texture residency and mip selection;
- copy / transfer path;
- explicit synchronization and presentation.

### Band B — optional accelerators
These may accelerate the runtime but may never become the only lawful path:
- tiled / sparse / reserved resources;
- descriptor indexing or large-table descriptor techniques;
- async compute specialization;
- sampler feedback or equivalent residency helpers;
- mesh / amplification shader families;
- work-graph / execution-graph style submission families;
- backend-specific submission or culling shortcuts with identical diagnostic publication.

### Band C — forbidden baseline assumptions
These may not be required for minimum shipping law or old-floor proof:
- mesh shaders as mandatory scene submission path;
- work graphs as baseline frame scheduler;
- one vendor-specific driver posture as the only legal path;
- one OS-release-specific feature as the only legal material-reveal path;
- strand-authoritative hair/fur rendering as the only legal coverage path.

## Capability-query law
At boot the backend layer must publish:
- backend class;
- driver family;
- feature-tier verdicts;
- optional-accelerator verdicts;
- substitutions and disabled fast paths.

No route may assume support for Band B features without explicit capability publication.

## Residency and streaming law
Sparse/tiled/reserved resource features are legal accelerators for large worlds.
They may reduce memory waste, enable remapping, and improve streaming posture.
They may not become hidden truth owners for whether a material, living-layer, or damage state exists.

## Submission and culling law
GPU-driven culling and indirect submission are legal accelerators.
They may reduce CPU scene-traversal cost.
They may not hide visibility reasons, material reasons, living-layer reasons, or degrade reasons from diagnostics.

## Certification and old-floor law
A certification pack must retain:
- backend class used;
- driver family used;
- feature-tier verdicts;
- first disabled optional accelerator that changed a measurable rung;
- proof that baseline truth remained lawful after fallback.

## Required companion docs
- root `20` and `42`
- engine `86–92`
- engine `104–106`
- engine constitutions for locked baseline tables
- editor `101–103`

## Current posture
`document_gold / closes_backend_class_and_feature_ladder_gap`

## Metal and platform-port law
`backend.metal_primary_apple` is a first-class runtime family, not an afterthought.
`backend.platform_port_native` is reserved for future platform-native ports that cannot be modeled as ordinary desktop backends.

Neither may rewrite renderer truth above the seam.
Both must publish the same diagnostic classes:
- backend class;
- shader target set;
- optional accelerators;
- disabled fast paths;
- capture legality verdict.

## Portability note
Cross-platform law is satisfied by stable contracts and explicit backend families.
It is not satisfied by collapsing every API into an over-wide pseudo-API with hidden costs.
