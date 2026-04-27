# Asset Import Processing And Canonicalization Route Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

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
