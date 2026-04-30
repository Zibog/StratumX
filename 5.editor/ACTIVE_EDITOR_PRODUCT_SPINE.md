# Active Editor Product Spine

This file records the crates that are genuinely wired into the runnable StratumX editor today.
Compilation, split structure, or roadmap intent do not make a crate product-ready on their own.

## Live product surfaces

- `6.apps/editor/stratumx_editor_app`
- `5.editor/l8.0-editor-shell`

## Live support crates wired into the current host/runtime path

- `5.editor/l7.0-editor-command-spine`
- `5.editor/l8.1-viewport-system`
- `5.editor/l8.10-diagnostics-surface`
- `5.editor/l9.0-world-authoring-suite`
- `5.editor/l9.2-terrain-landscape-authoring-suite`
- `5.editor/l9.6-weather-environment-authoring-suite`

## Future stubs and non-live editor crates

- `5.editor/editor-state-containers`
- `5.editor/l8.2-outliner-system`
- `5.editor/l8.3-content-browser-system`
- `5.editor/l8.4-inspector-system`
- `5.editor/l8.5-tool-context-system`
- `5.editor/l8.6-overlay-and-gizmo-system`
- `5.editor/l8.7-workspace-layout-system`
- `5.editor/l8.8-interaction-routing-system`
- `5.editor/l8.9-assistant-surface`
- `5.editor/l8.11-build-release-surface`
- `5.editor/l9.1-scene-entity-authoring-suite`
- `5.editor/l9.3-material-lookdev-authoring-suite`
- `5.editor/l9.4-destruction-fracture-authoring-suite`
- `5.editor/l9.5-simulation-ai-authoring-suite`
- `5.editor/l9.7-animation-cinematics-authoring-suite`
- `5.editor/l9.8-audio-voice-authoring-suite`
- `5.editor/l9.9-ui-hud-authoring-suite`
- `5.editor/l9.10-quest-event-logic-authoring-suite`
- `5.editor/l9.11-build-validation-release-suite`
- `5.editor/l10.0-project-bootstrap-service`
- `5.editor/l10.1-import-export-pipeline-service`
- `5.editor/l10.2-graph-authoring-service`
- `5.editor/l10.3-automation-and-batch-service`
- `5.editor/l10.4-script-and-hot-reload-service`
- `5.editor/l10.5-plugin-and-extension-host`
- `5.editor/l10.6-template-preset-and-scaffold-service`
- `5.editor/l10.7-package-market-and-dependency-service`
- `5.editor/l11.0-collaboration-session-surface`
- `5.editor/l11.1-review-annotation-surface`
- `5.editor/l11.2-asset-gate-and-approval-surface`
- `5.editor/l11.3-playtest-and-capture-operations`
- `5.editor/l11.4-production-dashboard-and-traceability`
- `5.editor/l11.5-learning-onboarding-and-help-surface`

## Not production-ready

- `5.editor/l8.1-viewport-system` is live as support, but the current app still renders a shell-level viewport placeholder instead of a full viewport-system-owned presentation path.
- `5.editor/l8.10-diagnostics-surface` is live as support, but the desktop host still owns thin diagnostics presentation glue.
- `5.editor/l9.0-world-authoring-suite` is live as host support for world lifecycle, not as a full product-complete authoring surface.
- `5.editor/l9.2-terrain-landscape-authoring-suite` is live as host support, but the app-side terrain panel remains a thin host-owned surface rather than a suite-owned production panel.
- `5.editor/l9.6-weather-environment-authoring-suite` is live as host support, but the app-side environment controls remain thin host glue.
- `5.editor/l8.5-tool-context-system`, `5.editor/l9.3-material-lookdev-authoring-suite`, and `5.editor/l10.0-project-bootstrap-service` have real code and completed phase splits, but they are not imported by the live `6.apps/editor/stratumx_editor_app/src/**` path.

## Rule

Crates not listed in the live sections above must not be described as product-complete or production-ready until they are imported into the active app/host path and pass the workspace quality gates.
