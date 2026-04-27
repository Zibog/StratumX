# scene_entity_authoring_suite Level

Canonical layer: `scene_entity_authoring_suite`
Activation class: `warm-suite`.

## Owns
- entity placement
- grouping and hierarchy workflows
- component authoring views
- prefab instance and override views
- transform and scene diagnostics

## Consumes
- scene/entity projections
- selection
- inspector
- viewport
- validation and preview hooks

## Emits
- scene/entity edit requests
- spawn/place/duplicate/group requests
- prefab apply/revert/unpack/create-variant requests
- component add/remove/edit requests

## Never owns
- entity truth
- prefab truth outside declared projections
