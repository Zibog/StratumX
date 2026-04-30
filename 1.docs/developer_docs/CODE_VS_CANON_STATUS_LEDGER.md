# Code vs Canon Status Ledger

## Overview

This ledger tracks the alignment between canonical documentation (canon) and actual code implementation for each major domain in the StratumX project. It provides visibility into which domains have comprehensive documentation but limited implementation (docs ahead), which have balanced progress (partial/strong), and which are in early stages (early).

The purpose of this ledger is to guide implementation priorities and ensure that the next development sprint focuses on domains where documentation is ready but code implementation lags behind.

## Status Definitions

- **docs ahead**: Documentation is substantially more complete than code implementation
- **partial**: Both documentation and code have moderate coverage, but significant work remains
- **strong**: Both documentation and code are well-developed
- **early**: Both documentation and code are in early stages
- **gold clean**: Domain has passed all validation gates and is production-ready

## Domain Status Table

| Domain | Docs Readiness | Code Readiness | Status | Next Implementation After Cleanup |
|--------|---------------:|---------------:|--------|-----------------------------------|
| Graphics / Native Graphics Port | 92% | 40% | docs ahead | Implement graphics port core/null/vulkan |
| Asset / Model Pipeline | 88% | 30% | docs ahead | Implement source registry and cooked .sx* pipeline |
| Materials | 92% | 58% | partial | Implement .sxmat, profile registry, shader variant key builder |
| Audio | 82% | 30% | docs ahead | Implement event/bank/mix backend seed |
| Netcode | 75% | 15% | early | Implement authority and replication skeleton |
| Editor | 88% | 60% | partial | Stitch active product spine and keep future surfaces honest |

## Domain Details

### Graphics / Native Graphics Port

**Docs Readiness:** 92%
**Code Readiness:** 40%
**Status:** docs ahead

The graphics domain has comprehensive canonical documentation covering the native graphics port architecture, packet command reference, and platform portability requirements. However, the code implementation is at 40%, with the core graphics port, null backend, and Vulkan backend requiring substantial implementation work.

**Next Steps:**
- Implement graphics port core abstraction layer
- Implement null backend for headless testing
- Implement Vulkan backend for production rendering

### Asset / Model Pipeline

**Docs Readiness:** 88%
**Code Readiness:** 30%
**Status:** docs ahead

The asset pipeline domain has strong documentation covering DCC source formats, cooked asset formats, and the import/cook/certification workflow. Code implementation is at 30%, with the source registry and cooked asset pipeline requiring significant development.

**Next Steps:**
- Implement source registry for tracking DCC assets
- Implement cooked .sx* pipeline for asset processing
- Build import/cook/certification tooling

### Materials

**Docs Readiness:** 92%
**Code Readiness:** 58%
**Status:** partial

The materials domain has excellent documentation and moderate code implementation. The material authoring suite exists with basic functionality, but the .sxmat format, profile registry, and shader variant key builder need completion.

**Next Steps:**
- Implement .sxmat file format and serialization
- Implement profile registry for material profiles
- Implement shader variant key builder for permutation management

### Audio

**Docs Readiness:** 82%
**Code Readiness:** 30%
**Status:** docs ahead

The audio domain has solid documentation covering event/bank/mix architecture and the audio authoring workflow. Code implementation is at 30%, with the event/bank/mix backend requiring foundational work.

**Next Steps:**
- Implement event/bank/mix backend seed
- Build audio authoring service integration
- Implement audio playback and mixing runtime

### Netcode

**Docs Readiness:** 75%
**Code Readiness:** 15%
**Status:** early

The netcode domain is in early stages for both documentation and implementation. The packet command reference and replication architecture are documented, but code implementation is minimal at 15%.

**Next Steps:**
- Implement authority and replication skeleton
- Build netcode packet serialization
- Implement client-server synchronization primitives

### Editor

**Docs Readiness:** 88%
**Code Readiness:** 60%
**Status:** partial

The editor domain has strong documentation and moderate implementation. The active product spine exists with world authoring, material authoring, terrain authoring, and weather authoring suites. The focus is on stitching the spine together and maintaining architectural discipline for future surfaces.

**Next Steps:**
- Stitch active product spine components
- Keep future surfaces honest (maintain as stubs)
- Ensure layer boundaries and architectural discipline

## Maintenance

This ledger should be updated:
- After each major implementation sprint
- When documentation coverage changes significantly
- When new domains are added to the project
- Before planning the next development phase

## Related Documents

- [Stack Map](../canonical/02_STACK_MAP.md)
- [Package Role Map](../canonical/03_PACKAGE_ROLE_MAP.md)
- [Global Dependency Model](../canonical/04_GLOBAL_DEPENDENCY_MODEL.md)
- [Codebase Build Map](./01_CODEBASE_BUILD_MAP.md)
