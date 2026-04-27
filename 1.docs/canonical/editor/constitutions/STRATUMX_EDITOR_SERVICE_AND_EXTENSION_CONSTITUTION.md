# STRATUMX_EDITOR_SERVICE_AND_EXTENSION_CONSTITUTION

## Scope
This constitution governs editor services, plugins, packages, automation, and extension points.

## Binding laws
- service and extension surfaces stay request/result oriented and may not own lower truth;
- plugin, package, importer, renderer, and command registration points must be explicit and auditable;
- extensions may add UI and behavior without seizing shell or authority ownership.

## Audit checks
- `L10` service contracts describe legal dependencies, invalidation, and budgets concretely;
- plugin and package services respect runtime/build/release boundaries;
- extension isolation and registration law remain aligned with evidence and readiness artifacts.
