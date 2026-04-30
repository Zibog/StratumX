# Asset Import Processing And Canonicalization Route Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

Asset import is lawful only when the route declares:
- source asset class;
- canonical schema revision;
- normalization rule id;
- artifact family;
- owning editor lab;
- proof-region placement scope;
- first focus target after success;
- first blocker after failure.

| Operator control surface | Route id cluster | SDK families | Required artifacts | First focus | Recovery |
|---|---|---|---|---|---|
| editor `107` import source payload | `route.content.import.*` | content import packets + `sdk/77` | raw ref, canonical ref, source lineage | owner lab asset panel | rerun normalization |
| editor `107` run processor profile | `route.content.processor.*` | content processing packets + `sdk/77` | processor profile ref, canonical digest, proof-region placement tags | inspector schema tab | restore prior revision |
| editor `107` assign runtime binding | `route.content.binding.*` | content binding packets + `sdk/77` | runtime binding artifact, identity lineage, drift refs | binding surface | drop one scope tier |
| editor `107` stage proof-region payload | `route.content.proof_region.*` | content staging packets + `sdk/77` | region recipe, staged asset set, missing-coverage digest | proof-region recipe board | remove unresolved dependency or re-stage |

No import route may silently normalize, silently rename pack ids, or publish terminal success without artifact refs and proof-region placement scope.

---

# V33 asset/model practical closure assimilation

Stack version: `SX-CANON/1.0.27/STACK-v33`

## Asset/model practical closure

The import route must be explicit for static meshes, skeletal meshes, terrain tiles, foliage, groom/fur coverage, collision, LOD, proxy, impostor, materials, textures, audio banks, and animation clips.

Import is not complete when a file appears in the content browser. Import is complete only when the asset has canonical identity, validation verdict, runtime binding package, failure diagnostics, editor preview, and cook/freeze eligibility.
