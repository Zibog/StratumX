# STRATUMX_SDK_L5_SEMANTIC_SEALING_CONSTITUTION

## Scope
This constitution keeps bridge semantics sealed against upward policy drift.

## Binding laws
- `L5` exports facts that are already typed and closed enough to consume safely;
- semantic widening must happen above the bridge;
- the bridge may normalize forms but may not introduce new authoring meaning.

## Audit checks
- any semantic enrichment proposal is justified at package level;
- local packs describe normalization, not policy invention;
- upper layers do not depend on hidden bridge interpretation.
