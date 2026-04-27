# animation_cinematics_authoring_suite Level

Canonical layer: `animation_cinematics_authoring_suite`
Activation class: `warm-suite`.

## Owns
- timeline/sequencer surfaces
- camera tools and shot management
- event, dialogue, audio, and gameplay signal track views
- animation diagnostics and preview controls

## Consumes
- animation/cinematic projections
- viewport previews
- asset projections
- validation hooks

## Emits
- animation/cinematic edit requests
- shot and track requests
- preview and bind-validation requests

## Never owns
- animation truth
- runtime playback truth
