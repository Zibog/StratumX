# Troubleshooting: DCC Import, Plugin Conversion, and Cooked Asset Failures

Stack version: `SX-CANON/1.0.28/STACK-v34`

Role: **troubleshooting**.

## Symptom: `.blend` does not import

Likely causes:

- Blender bridge not configured.
- Blender executable missing.
- unsupported add-on data.
- export script failed.

Recovery:

1. Open Import Report.
2. Check bridge path.
3. Run bridge doctor.
4. Export `.glb/.fbx/.usd` manually as fallback.
5. Reimport with identity preservation.

## Symptom: `.max` does not import

Likely causes:

- 3ds Max not installed.
- batch bridge unavailable.
- plugin dependency missing.
- source file version mismatch.

Recovery:

1. Open 3ds Max bridge diagnostics.
2. Export FBX/USD/glTF/OBJ from 3ds Max.
3. Disable unsupported plugin conversion or bake it.
4. Reimport.
5. Keep source provenance.

## Symptom: material looks wrong

Likely causes:

- unsupported DCC shader node.
- color space mismatch.
- missing texture.
- spec/gloss conversion ambiguity.
- normal map convention mismatch.

Recovery:

1. Open material conversion report.
2. Assign StratumX hybrid material family.
3. Rebind texture channels.
4. Recalculate normal convention.
5. Capture material preview.

## Symptom: cooked asset is not runtime-ready

Likely causes:

- missing collision decision.
- invalid skeleton.
- missing texture dependency.
- no material slot assignment.
- unsupported procedural geometry not baked.

Recovery:

1. Open Cook Report.
2. Resolve blockers one by one.
3. Re-run validation.
4. Re-cook.
5. Confirm runtime package id.

## Mandatory blocker behavior

Import tools must never say success when runtime cook failed. They must publish exact blocker codes and next legal actions.
