# Scope

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

This package defines the upper tools stack above SDK `L5`.
It freezes:
- `L6` as the only editor authoring authority owner;
- `L6A` as the only assistant runtime bridge;
- `L7` as cold studio orchestration and governance;
- `L7A` as cold assistant planning and reasoning;
- plane-separated data flow for authoring, preview, validation, build, release, diagnostics, and runtime attach;
- memory, GPU, disk, and degradation discipline for a large world editor.

This package does not define:
- engine internals;
- `L5` bridge internals;
- project-specific game rules;
- model-vendor specifics;
- panel chrome or editor-shell widget layout.

## Dream-editor posture
The tooling package must be strong enough to back:
- large-world outliner/data-layer/content workflows;
- prefab and override legality;
- background asset processing and reimport;
- validation, bake, cook, build, and release;
- runtime inspection and play-in-editor attachment;
- plugin and package ecosystems.

It must do this without turning itself into one monolithic hidden store.
