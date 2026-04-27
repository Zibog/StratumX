# Thin RHI And Backend Seam Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the engine-owned graphics seam that sits above Direct3D 12 and Vulkan backends without turning the canon into a backend-specific religion.

This document exists so the archive answers the practical question cleanly:
what exactly is allowed between engine render truth and native graphics APIs?

## Core law
The engine must own one thin runtime hardware interface.
It is not a fake universal GPU API and not a translation hack.
It is the minimal canonical seam that:
- exposes the common minimum needed by the renderer and frame graph;
- publishes backend posture and capability verdicts;
- allows optional backend-specific acceleration through explicit escape hatches;
- keeps diagnostics and certification comparable across backends.

## Canonical backend modules
| Backend module | Status | Role |
|---|---|---|
| `rhi.backend.dx12` | first-class | primary Windows runtime backend |
| `rhi.backend.vulkan` | first-class | portable parity backend and locked benchmark backend family when configured |
| `rhi.backend.metal` | first-class | Apple-platform native runtime backend |
| `rhi.backend.platform_port.*` | reserved first-class family | platform-native port adapter family for restricted platforms |
| `rhi.backend.null` | required | headless validation / non-presenting proof path |

## Frontend contract law
The engine-facing RHI contract must be backend-neutral for the following surfaces:
- device and queue identity publication;
- resource creation and lifetime;
- transfer / copy / upload;
- pass execution and barriers;
- descriptor / binding publication;
- pipeline identity and cache posture publication;
- presentation / no-presentation verdict;
- timing-query posture publication.

The frontend contract may not expose backend-specific vocabulary as its only legal language.

## Native escape-hatch law
Backend-specific escape hatches are legal.
They must be:
- explicitly typed;
- capability-gated;
- optional;
- observable in diagnostics.

An escape hatch may accelerate one path.
It may not silently redefine truth ownership, certification output, or old-floor legality.

## Shader-target law
The canonical shader authoring source is HLSL-first.
The backend seam must support at minimum:
- DXIL generation for `rhi.backend.dx12`;
- SPIR-V generation for `rhi.backend.vulkan`.

Backend-specific shader specialization is legal only when the common minimum path remains available and the selected variant is published in runtime diagnostics.

## Pipeline and descriptor law
Pipelines, resource layouts, and descriptor groupings must be authored in engine-facing canonical forms first.
Backend lowering may specialize layout packing or binding implementation.
It may not invent hidden resource ownership or silently drop a declared material or coverage binding.

## Present and non-present law
The seam must support both:
- presenting realtime products;
- non-presenting validation/certification flows.

A non-presenting path may skip display output.
It may not skip frame-truth publication, timing legality checks, or artifact capture when those are required by the active route.

## Diagnostics law
Every run must be able to publish:
- active backend module;
- active backend class;
- shader target set;
- optional-accelerator verdicts;
- first substitution or disabled fast path;
- native escape hatches that were activated.

## Forbidden shortcuts
- hiding backend choice inside build flavor with no runtime publication;
- one translation layer as the only legal path to a backend;
- backend-specific shader source as the only legal source language;
- letting a backend escape hatch become the only working path for coverage, material reveal, or certification capture.

## Required companion docs
- engine `103`
- engine `86–92`
- root `42`
- editor `101–103`

## Current posture
`document_gold / closes_thin_rhi_backend_seam_gap`

## Platform adapter law
A platform-native port adapter is legal when the platform requires its own backend family.
The adapter must still implement the engine-facing seam for:
- resource creation and lifetime publication;
- pass execution and synchronization publication;
- present or no-present verdict publication;
- diagnostics and capture legality publication.

## No-fake-universal-API law
The thin RHI is not allowed to swell into a fake “all graphics APIs are the same” layer.
Its job is to stabilize the frontend contract, not to erase native differences.
