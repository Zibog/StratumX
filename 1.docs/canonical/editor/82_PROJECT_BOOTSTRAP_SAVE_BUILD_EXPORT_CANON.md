# Project Bootstrap Save Build Export Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Production buttons

| Button | Purpose | Route | Next focus | Recovery | Relay pack |
|---|---|---|---|---|---|
| `btn.project.bootstrap` | create project and seed proof-region recipe | `route.project.bootstrap` | workspace shell | rerun template select | `pack.brutal_proof_region_relay` |
| `btn.workspace.save_profile` | persist workspace/save profile | `route.workspace.save_profile` | save profile panel | restore last-known schema | `pack.brutal_proof_region_relay` |
| `btn.build.package` | package build from retained bundle only | `route.build.package` | build result panel | recover build baseline | `pack.brutal_proof_region_relay` |
| `btn.export.target` | export selected target | `route.export.target` | export artifact panel | fix target profile | `pack.brutal_proof_region_relay` |
| `btn.launch.verify_first_result` | launch and verify first result | `route.launch.verify_first_result` | launch diagnostics panel | restore launch baseline | `pack.brutal_proof_region_relay` |

No bootstrap/build/export action may bypass artifact, trace, freeze relevance, proof-region recipe, or retained baseline fields.
