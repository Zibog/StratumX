# world_authoring_suite Level

Canonical layer: `world_authoring_suite`
Activation class: `warm-suite`.

## Owns
- world partition surfaces
- region and zone authoring views
- streaming cell tools
- data-layer editing views
- world diagnostics and placement flows

## Consumes
- world projections
- viewport system
- outliner/data-layer views
- preview and validation hooks

## Emits
- world partition requests
- region/cell load-preview requests
- data-layer assignment/toggle requests
- world diagnostics and validation requests

## Never owns
- world truth
- streaming runtime truth
