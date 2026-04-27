# Editor Boot and Level Space Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the editor product posture from process launch to first useful world-space interaction.
The target is not a blank shell.
The target is a world-ready editor that opens into a legal level-authoring space with an active viewport, a loaded world, visible terrain, visible environment response, and diagnostics-ready proof surfaces.

## Canonical target
The canonical target posture is:
`launch canonical host -> boot editor shell -> restore or choose legal workspace -> open startup/reference world -> mount primary viewport -> attach legal projections -> begin level-space authoring`.

## Exact closure question
The thread is closed only when one operator can:
1. launch the one canonical editor host;
2. land in a restored or default world-authoring workspace;
3. see a primary level-space viewport immediately;
4. see the loaded startup/reference world without manual panel recovery;
5. see legal terrain and legal environment response in that viewport when the world has those bindings;
6. inspect world, content, details, diagnostics, and runtime posture from anchored surfaces;
7. begin editing without first constructing a temporary preview scene by hand.

## World role definitions
- `startup world`: the world the editor opens by policy when no stronger restore source exists.
- `reference world`: a world used to validate the first product-result loop.
- `demo world`: a world used for showcase routes; it may reuse the same lower-stack path but does not define editor identity.

These roles may resolve to the same world in early development, but they are not the same concept.

## Mandatory boot law
On successful boot, the editor must choose exactly one of the following legal outcomes:
1. restore the last valid workspace and last valid world;
2. restore the last valid workspace and fall back to the configured startup/reference world;
3. if no prior workspace exists, open the configured default workspace with the configured startup/reference world;
4. explicit world-open failure posture.

The product must never boot into an empty shell if a legal startup/reference world is configured.

## Mandatory visible anchors on boot
The default level-space boot must surface:
- one focused primary viewport
- outliner / scene tree
- content browser / filesystem
- details / inspector
- diagnostics anchor
- toolbar/status strip with play/simulate/build posture
- active workspace and world identifier

## Canonical boot flow
### Host layer
The canonical host may own:
- process bootstrap
- platform window bootstrap
- renderer/device bootstrap
- editor package mount
- project selection and startup-world handoff

The host must not own:
- panel composition truth
- viewport camera truth after mount
- world-authoring truth
- environment authoring truth
- parallel preview/editor chrome

### Editor layer
`editor/` owns:
- boot layout restoration
- level-space surface composition
- viewport presentation state
- selected world/object presentation
- panel activation posture
- boot error presentation

### Tooling layer
`tooling/` owns:
- workspace coordination and restored-session legality
- preview/runtime projections
- validation projections
- build/release projections
- legal world-open requests
- legal preview attach for terrain, environment, and runtime proof

### SDK layer
`sdk/` owns bridge-safe packets, observations, refs, cursors, and immutable bridge facts.
It does not own workspace, viewport, or startup-world UI truth.

### Engine layer
`engine/` owns runtime world truth, startup bind truth, terrain truth, environment truth, and legal imaging truth for the viewport result.

## Failure posture
If world-open or preview attach fails, the editor must still boot into a controlled failure posture.
That posture must show:
- boot stage that failed
- world or workspace identifier involved
- actionable recovery path
- diagnostics anchor for details

Silent fallback to an unrelated world is forbidden.
