# Asset Import to Scene Placement Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the first honest content workflow:
`import asset -> inspect/list asset -> place asset into scene -> bind transform/material state -> inspect the result`.

## Workflow law
The workflow is not closed if the stack can import a file but cannot legally explain:
- where the imported product lives;
- whether the product is merely metadata, a durable artifact, or a runtime-consumable asset;
- how placement into scene truth occurs;
- how diagnostics explain import or placement failure.

## Canonical route
`content browser/import surface -> tooling content import intent -> asset metadata/artifact result -> scene placement ask -> tooling scene authority -> sdk packet surfaces only when lower truth must be bound -> scene/outliner/inspector projections`

## Required user-visible steps
1. import a source file;
2. see the imported asset in the content browser;
3. inspect asset details and type;
4. place an instance into the scene;
5. inspect transform/material slots;
6. see diagnostics for success, denial, or degrade.

## Current implementation posture
The uploaded code proves:
- authoring-level `ImportStaticMesh` and `ImportTexture` commands;
- asset list/details DTOs;
- scene entity creation from assets inside tooling-local authoring state.

The uploaded code does not yet prove:
- durable artifact generation;
- reimport/migration closure;
- lower authoritative scene/world bind beyond the tooling session.

Therefore current posture is `partial`.
