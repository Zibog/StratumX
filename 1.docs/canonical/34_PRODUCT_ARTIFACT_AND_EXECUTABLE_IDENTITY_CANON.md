# Product Artifact And Executable Identity Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze how a built product is identified and how artifacts are traced back to the route that produced them.

## Artifact identity schema
| Field | Meaning | Rule |
|---|---|---|
| artifact_id | stable artifact identity | required |
| artifact_family | build, package, symbol, manifest, capture, report | required |
| build_profile_ref | profile used to create the artifact | required |
| project_id | project that owns the artifact | required |
| workspace_id | workspace that emitted the artifact | required |
| proof_region_recipe_ref | staged brutal proof-region recipe that fed the build | required for phase-5 closure |
| content_snapshot_ref | canonical content snapshot | required |
| world_snapshot_ref | world snapshot | required |
| gameplay_snapshot_ref | gameplay snapshot | required |
| restore_snapshot_ref | save/load/restore anchor used for replay-safe proof | required when persistence is part of the bundle |
| mixed_pack_digest_ref | retained digest for active mixed packs | required when certification touched mixed packs |
| executable_signature_ref | signature or launch fingerprint | required for launchable outputs |
| first_result_verification_ref | retained launch verification artifact | required for launchable outputs |
| certification_state | uncertified, provisional, certified, frozen | required |

## Traceability law
Every launchable artifact must answer:
- what project made me;
- what proof-region recipe made me;
- what snapshots made me;
- what restore anchor and mixed-pack digest apply to me;
- what profile made me;
- what compare baseline applies to me;
- whether certification approved me;
- and which first-result verification artifact proved that the build actually launched.
