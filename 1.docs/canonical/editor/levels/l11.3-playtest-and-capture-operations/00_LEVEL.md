# L11.3 Playtest and Capture Operations

## Role

Own play-in-editor control, simulation stepping, runtime attach, runtime inspection publication,
and capture/export operations for validation and review.

## Owns

- play in editor session lifecycle
- simulate/step/pause/stop commands
- runtime entity bind and detach
- runtime inspection publication
- authoring-to-runtime property sync envelopes
- capture jobs for screenshots, clips, traces, and diagnostics bundles

## Must Not Own

- editor shell widget ownership
- authoritative asset build ownership
- validation rule definition ownership
- package registry ownership

## External Dependencies

- editor:l8.1-viewport-system
- editor:l8.4-inspector-system
- editor:l8.8-console-notification-system
- tooling:l6.11-validation-runtime
- tooling:l6.12-preview-runtime
- tooling:l6.13-build-runtime
- tooling:l6.14-release-runtime
- sdk:l5 runtime/public handles
