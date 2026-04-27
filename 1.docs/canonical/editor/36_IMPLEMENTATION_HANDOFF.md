# Editor Implementation Handoff

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines the implementation handoff package for engineers and agents implementing the editor product.
The goal is not “many panels”.
The goal is one honest authoring product spine.

## Build target
Implement a production-grade game editor that can actually assemble and enter a world:
- one canonical host
- shell, viewport, outliner, content browser, inspector, diagnostics rail
- legal world-open / restore / fallback path
- legal terrain and environment delivery into one viewport
- play/simulate and grounded walk in the same world
- return to authoring without world drift
- strict command, dto, and identity discipline

## Lower-stack prerequisites
The editor handoff assumes:
- `sdk/L5` remains bridge-only and non-editor-shaped;
- `tooling/L6` owns authority, validation, preview, build, and release;
- `tooling/L6.10 workspace_runtime` remains session-safe coordination only and does not own product UI;
- `engine` owns world truth, terrain truth, environment truth, runtime truth, and imaging extraction.

## Critical documents
### Root and normalization
- `../10_FIRST_PRODUCT_RESULT_CANON.md`
- `../11_CANON_INPUT_AND_NORMALIZATION_RULES.md`
- `../12_CANON_REPO_ALIGNMENT_AND_REMEDIATION_CANON.md`

### Product shell and viewport spine
- `08_EDITOR_PRODUCT_MODEL.md`
- `10_VIEWPORT_AND_NAVIGATION_MODEL.md`
- `20_BUILD_RELEASE_DIAGNOSTICS_SURFACE_MODEL.md`
- `22_EDITOR_DATAFLOW_AND_ACTIVATION_MODEL.md`
- `31_SHARED_TYPE_REGISTRY.md`
- `44_SKY_AND_ENVIRONMENT_VIEWPORT_DELIVERY_CANON.md`
- `45_EDITOR_BOOT_AND_LEVEL_SPACE_CANON.md`
- `46_ENGINE_TO_VIEWPORT_FRAME_DELIVERY_CANON.md`
- `47_EDITOR_CONTROL_SURFACE_AND_COMMAND_CANON.md`
- `48_EDITOR_HOST_AND_PRODUCT_SPINE_CANON.md`

### Domain closure anchors
- `16_OUTLINER_WORLD_BROWSER_MODEL.md`
- `17_INSPECTOR_AND_DETAILS_MODEL.md`
- `18_PLAY_SIMULATE_DEBUG_MODEL.md`
- `21_DOMAIN_SUITE_MODEL.md`
- `49_INTERACTION_DRIVEN_MOTION_AND_CONSTRAINT_CANON.md`

## Implementation law
If a feature seems easier to build by storing extra hidden truth in a panel, preview package, or mock backend, the feature is being built in the wrong layer.

## Handoff emphasis
Before broad editor dressing, close one real world loop.
If the editor can boot, open a legal world, show terrain and environment in one honest viewport, and support play/walk/return with diagnostics, the product has a spine.
If it cannot, more docks and chrome only decorate the absence.
