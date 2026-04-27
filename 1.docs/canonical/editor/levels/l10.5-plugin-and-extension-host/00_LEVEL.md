# L10.5 Plugin and Extension Host

## Role

Own the canonical runtime contract for editor extensibility. This level hosts dock plugins,
inspector renderers, component editors, asset importers, validation rules, commands,
timeline tracks, viewport overlays, and context-menu extensions.

## Owns

- plugin registration lifecycle
- plugin capability descriptors
- extension mount scopes
- sandbox and trust posture for extensions
- registration conflict handling
- extension health and disablement status
- plugin-visible command and panel publication surfaces

## Must Not Own

- product shell layout ownership
- viewport rendering ownership
- runtime simulation ownership
- asset build execution ownership
- package resolution ownership

## External Dependencies

- L8 editor shell for mount points and widget host surfaces
- L8 inspector/outliner/content systems for target integration slots
- tooling validation_runtime for rule execution
- tooling preview_runtime for preview-only extension outputs
- tooling build_runtime for importer/postprocessor dispatch
- tooling release_runtime for extension packaging outputs
- tooling L6A assistant runtime for assistant-facing extension discovery
