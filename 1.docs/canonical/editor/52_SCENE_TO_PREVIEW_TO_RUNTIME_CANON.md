# Scene to Preview to Runtime Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the editor-facing workflow that connects authored scene state to preview/runtime posture and back.
It is the user-facing counterpart to the product spine and runtime-entry law.

## Canonical route
`scene/outliner/inspector state -> tooling scene authority -> preview/runtime entry request -> sdk control/observation surfaces -> engine/runtime attach or startup bind -> runtime diagnostics/observations -> editor runtime surfaces -> return to authoring`

## Required distinctions
The editor must clearly distinguish:
- scene authoring truth;
- preview-only posture;
- simulate/play runtime posture;
- returned/restored authoring posture.

## Required visible states
- `PreviewReady`
- `PreviewDenied`
- `SimulateAttached`
- `PlayAttached`
- `RuntimePaused`
- `ReturnToAuthoring`
- `AuthoringRestored`
- `RuntimeDegraded`

## Current implementation posture
The uploaded code proves:
- a vertical-slice runtime session;
- preview state DTOs in tooling authoring session;
- a Rust-native host and a Tauri bridge that can bootstrap a demo runtime path.

The uploaded code does not yet broadly prove:
- same-world runtime attach for general scenes;
- authoritative restore semantics for arbitrary authoring state;
- general play/simulate beyond the vertical slice.

Current posture: `partial`.
