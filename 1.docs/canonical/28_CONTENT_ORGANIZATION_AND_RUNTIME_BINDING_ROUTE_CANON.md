# Content Organization And Runtime Binding Route Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

Content organization and runtime binding form the lawful handoff between imported assets and runtime-owned truth.

| Route id cluster | Editor controls | SDK families | Engine targets | Required refs | Denials |
|---|---|---|---|---|---|
| `route.content.organize.*` | editor `107` package / identity organization controls | content organization packets + `sdk/77` | owning engine package / identity truth | package ref, layer ref, identity ref | invalid scope, missing package |
| `route.content.bind_material.*` | `btn.render.material.bind_shader_variant` plus editor `107` binding controls | render material packets + `sdk/77` | render material truth | material profile ref, shader stack ref | shader schema mismatch |
| `route.content.bind_audio.*` | `btn.audio.bind_zone_mix` plus editor `107` binding controls | audio packets + `sdk/77` | audio emitter / zone-mix truth | emitter ref, zone ref, bus ref | missing zone / bus |
| `route.content.bind_heavy_profile.*` | owning domain rows in `editor/110` | domain packets + `sdk/77` | heavy truth owner | profile ref, artifact ref | baseline missing, wrong revision |
| `route.content.bind_proof_region.*` | editor `107` proof-region binding controls | binding packets + `sdk/77` | mixed domain owners listed by staged recipe | region recipe ref, runtime binding graph, unresolved blocker digest | missing required family, incompatible staged identity |

Every binding route must preserve focus target, next legal recovery action, disabled reason family, and proof-region recipe backlink.
