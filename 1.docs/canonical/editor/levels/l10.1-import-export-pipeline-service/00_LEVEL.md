# import_export_pipeline_service Level

Canonical layer: `import_export_pipeline_service`
Activation class: `batch-service`.

## Owns
- asset-processor dashboard UI
- import/export form state
- selected profiles and presets
- remediation side-panel state
- export routing and recent-task view state

## Consumes
- build-runtime queue state
- validation findings
- artifact and dependency projections
- release/runtime export status where legal

## Emits
- import/reimport/rebuild/export requests
- validation/dependency scan requests
- open-source/reveal-generated-file intents
- bundle/addressable/streamable intents

## Never owns
- asset authority
- asset processor truth
- build or release queue truth
