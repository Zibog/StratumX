# STRATUMX_SDK_L5_RESULT_ARTIFACT_VERDICT_SEPARATION

## Scope
This constitution enforces the separation between result rows, artifact rows, and verdict rows.

## Binding laws
- result rows report what happened;
- artifact rows identify durable produced outputs;
- verdict rows classify legality/compatibility/approval posture;
- no row may silently play two of these roles at once.

## Audit checks
- result/artifact/verdict registries remain distinct;
- evidence and local packs use the correct class names;
- downstream tooling can route these classes without reinterpretation.
