# First Playable Slice Conveyor And Proof Region Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the route mesh for the narrow proof region from empty project to retained proof bundle.

## Required routes
- `route.proof_region.bootstrap.v1`
- `route.proof_region.bind_world_materials.v1`
- `route.proof_region.prepare_tunnel_scene.v1`
- `route.proof_region.run_visual_audio_truth.v1`
- `route.proof_region.capture_bundle.v1`
- `route.proof_region.review_readiness.v1`

## Stage law
`project create -> world open -> terrain/material bind -> sky/time/weather bind -> tunnel/basement proof pocket prep -> destruction/audio/light proof -> capture bundle -> readiness review`

## Required artifacts
Each stage must preserve:
- proof-lane id;
- first blocker family;
- focused recovery target;
- retained artifact refs once capture begins.
