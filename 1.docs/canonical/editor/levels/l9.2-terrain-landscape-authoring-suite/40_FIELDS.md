# Fields

## Canonical owned records
| Field | Type | Role | Invariant |
|---|---|---|---|
| terrain_suite_session_id | TerrainSuiteSessionId | active terrain authoring session | unique per terrain context |
| terrain_region_ref | TerrainRegionRef | current terrain region under edit | explicit and bounded |
| brush_state | TerrainBrushState | active terrain brush configuration | editor-local and explicit |
| layer_paint_state | TerrainLayerPaintState | active terrain layer paint posture | finite enum only |
| terrain_build_request_ref | TerrainBuildRequestRef | pending terrain build/bake request | must remain command-visible |

## Field law
The records above are the minimum editor-owned state needed to drive `terrain_landscape_authoring_suite` without stealing truth from neighboring levels or lower packages.
