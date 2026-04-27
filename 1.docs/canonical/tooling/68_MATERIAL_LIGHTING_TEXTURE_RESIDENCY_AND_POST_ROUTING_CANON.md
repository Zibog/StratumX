# Material Lighting Texture Residency And Post Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact routing law for material/shader/light/environment/residency/post interactions visible to the operator shell.

## Intent families
- `intent.render.material_bind`
- `intent.render.environment_bind`
- `intent.render.residency_inspect`
- `intent.render.post_profile_change`

## Required routing fields
- target material or environment ref
- route-local legality verdict
- required lower-surface packets
- invalidation scope
- focus target on success/failure
- retained artifact policy

## Route-local laws
- material changes must invalidate only the declared preview scopes;
- environment changes must not silently reset camera/exposure history unless declared;
- residency inspection is read-only and must surface fallback/substitution classes;
- post-profile changes must publish exposure/post lineage.
