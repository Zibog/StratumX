# Editor Test Class Registry v2

## Purpose
This document registers all required test classes for editor package gold closure.

## Test Class Registry

| Test ID | Test Class | Primary Surfaces | Blocking Docs | Status |
|---------|------------|------------------|---------------|--------|
| TEST-ED-001 | Shell composition tests | `L8.0` shell | `../../08_EDITOR_PRODUCT_MODEL.md`, `../../33_BOUNDARY_PRESERVATION_MATRIX.md` | registered |
| TEST-ED-002 | Viewport/navigation/manipulator tests | `L8.1` viewport | `../../10_VIEWPORT_AND_NAVIGATION_MODEL.md`, `../../11_SELECTION_AND_INTERACTION_MODEL.md` | registered |
| TEST-ED-003 | Outliner contract tests | `L8.2` outliner | `../../16_OUTLINER_WORLD_BROWSER_MODEL.md`, `../../33_BOUNDARY_PRESERVATION_MATRIX.md` | registered |
| TEST-ED-004 | Content-browser contract tests | `L8.3` content browser | `../../15_CONTENT_AND_ASSET_MODEL.md`, `../../33_BOUNDARY_PRESERVATION_MATRIX.md` | registered |
| TEST-ED-005 | Inspector/component contract tests | `L8.4` inspector | `../../17_INSPECTOR_AND_DETAILS_MODEL.md`, `../../33_BOUNDARY_PRESERVATION_MATRIX.md` | registered |
| TEST-ED-006 | Panel anchor and command palette tests | `L8` panels | `../../13_PANEL_AND_VIEW_MODEL.md`, `../../14_COMMAND_PALETTE_AND_SHORTCUT_MODEL.md` | registered |
| TEST-ED-007 | Prefab/variant/override tests | scene + inspector | `../../17_INSPECTOR_AND_DETAILS_MODEL.md`, `../../28_ASSET_VERSIONING_AND_VARIANT_MODEL.md` | registered |
| TEST-ED-008 | Data-layer/world-partition/chunk tests | world suite + outliner | `../../16_OUTLINER_WORLD_BROWSER_MODEL.md`, `../../21_DOMAIN_SUITE_MODEL.md`, `../../26_SOURCE_CONTROL_AND_CONFLICT_MODEL.md` | registered |
| TEST-ED-009 | Asset processor/import/reimport tests | `L10.1` import/export | `../../15_CONTENT_AND_ASSET_MODEL.md`, `../../20_BUILD_RELEASE_DIAGNOSTICS_SURFACE_MODEL.md` | registered |
| TEST-ED-010 | Validation/bake/build/release surface tests | diagnostics/build surfaces | `../../20_BUILD_RELEASE_DIAGNOSTICS_SURFACE_MODEL.md`, `../../33_BOUNDARY_PRESERVATION_MATRIX.md` | registered |
| TEST-ED-011 | Runtime bridge / PIE / runtime inspector tests | `18` + `L11.3` | `../../18_PLAY_SIMULATE_DEBUG_MODEL.md`, `../../20_BUILD_RELEASE_DIAGNOSTICS_SURFACE_MODEL.md` | registered |
| TEST-ED-012 | Plugin capability and isolation tests | `L10.5` plugin host | `../../31_SHARED_TYPE_REGISTRY.md`, `../../32_FORBIDDEN_CONNECTIONS.md`, `../../33_BOUNDARY_PRESERVATION_MATRIX.md` | registered |
| TEST-ED-013 | Package/dependency service tests | `L10.7` package service | `../../20_BUILD_RELEASE_DIAGNOSTICS_SURFACE_MODEL.md`, `../../33_BOUNDARY_PRESERVATION_MATRIX.md` | registered |
| TEST-ED-014 | Source control and chunk-save tests | content/world surfaces | `../../26_SOURCE_CONTROL_AND_CONFLICT_MODEL.md` | registered |
| TEST-ED-015 | Undo/redo/history tests | all mutation surfaces | `../../24_UNDO_REDO_AND_HISTORY_MODEL.md` | registered |
| TEST-ED-016 | Autosave/recovery/restore tests | all surfaces | `../../25_AUTOSAVE_RECOVERY_AND_RESTORE_MODEL.md` | registered |
| TEST-ED-017 | Workspace schema and migration tests | shell/layout | `../../27_WORKSPACE_SCHEMA_AND_MIGRATION_MODEL.md` | registered |
| TEST-ED-018 | Activation/resource/budget tests | hot surfaces | `../../22_EDITOR_DATAFLOW_AND_ACTIVATION_MODEL.md`, `../../29_EDITOR_BUDGET_RUNTIME_MODEL.md`, `../../30_EDITOR_ACTIVATION_AND_COLD_SURFACE_MODEL.md` | registered |
| TEST-ED-019 | Hidden state audit tests | all surfaces | `../../32_FORBIDDEN_CONNECTIONS.md`, `../../33_BOUNDARY_PRESERVATION_MATRIX.md`, `../../34_GLOSSARY.md` | registered |
| TEST-ED-020 | Collaboration/production non-authority tests | `L11` surfaces | `../../32_FORBIDDEN_CONNECTIONS.md`, `../../33_BOUNDARY_PRESERVATION_MATRIX.md` | registered |

## Test Class Closure
All 20 required test classes are registered.
Each test class is bound to primary surfaces and blocking docs.
