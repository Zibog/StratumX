# Editor Button To Tooling To SDK To Engine Truth Chain Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Define the singular downward chain from one operator-visible concrete `btn.*` command to one legal engine truth owner.

## Exact lifecycle vocabulary
Every promoted row summarized here inherits the lifecycle contract from sdk `77`.

| route outcome class | exact allowed lifecycle |
|---|---|
| denied before execution | `ack.denied` |
| accepted success | `ack.accepted -> progress.bound? -> progress.running* -> partial_result* -> terminal_success` |
| accepted retryable failure | `ack.accepted -> progress.bound? -> progress.running* -> partial_result* -> retryable_failure` |
| accepted terminal failure | `ack.accepted -> progress.bound? -> progress.running* -> terminal_failure` |

`partial_result` is optional and may appear only if the operator receives a stable, visible intermediate artifact.
`ack.denied` may not be followed by any progress or terminal state.

## Downward chain clusters
| execution cluster | owner editor lab | tooling route family | sdk packet family | engine truth owner | allowed mutation class | denial family | command monitor duty |
|---|---|---|---|---|---|---|---|
| hydrology concrete rows in `editor/110` | `editor/66` | `route.hyd.*` | `packet.hydrology.*` | `engine/61` | author/bind/inspect/simulate/compare/capture/recover/certify | `HYD_*` | show route id, packet id, baseline id, artifact ref, focus target |
| storm and celestial-weather concrete rows in `editor/110` | `editor/67` | `route.storm.*` | `packet.storm.*` | `engine/49` + `engine/62` | author/bind/inspect/simulate/compare/capture/recover/certify | `STM_*`, `FIR_*` | show route id, packet id, front-track trace, artifact ref, focus target |
| soft-surface concrete rows in `editor/110` | `editor/68` | `route.soft.*` | `packet.soft.*` | `engine/63` | author/bind/inspect/simulate/compare/capture/recover/certify | `SFT_*` | show route id, packet id, contact trace, artifact ref, focus target |
| society concrete rows in `editor/110` | `editor/69` | `route.soc.*` | `packet.soc.*` | `engine/64` | author/bind/inspect/simulate/compare/capture/recover/certify | `SOC_*` | show route id, packet id, reason chain, artifact ref, focus target |
| tactics concrete rows in `editor/110` | `editor/70` | `route.tac.*` | `packet.tac.*` | `engine/65` | author/bind/inspect/simulate/compare/capture/recover/certify | `TAC_*` | show route id, packet id, cover/collapse reason chain, artifact ref, focus target |
| ecology concrete rows in `editor/110` | `editor/71` | `route.eco.*` | `packet.eco.*` | `engine/66` | author/bind/inspect/simulate/compare/capture/recover/certify | `ECO_*` | show route id, packet id, migration trace, artifact ref, focus target |
| wound concrete rows in `editor/110` | `editor/72` | `route.wound.*` | `packet.wound.*` | `engine/67` + `engine/71` | author/bind/inspect/simulate/compare/capture/recover/certify | `WND_*` | show route id, packet id, wound trace, artifact ref, focus target |
| semantic concrete rows in `editor/110` | `editor/73` | `route.semantic.*` | `packet.semantic.*` | `engine/68` | author/bind/inspect/simulate/compare/capture/recover/certify | `SEM_*` | show route id, packet id, grounding trace, artifact ref, focus target |
| world-floor concrete rows in `editor/110` | `editor/74` | `route.worldfloor.*` | `packet.worldfloor.*` | `engine/69` + `engine/98` + `engine/102` | author/bind/inspect/simulate/compare/capture/recover/certify | `FLR_*` | show route id, packet id, degrade rung, artifact ref, focus target |
| animation-synthesis concrete rows in `editor/110` | `editor/75` | `route.anim.*` | `packet.anim.*` | `engine/70` | author/bind/inspect/simulate/compare/capture/recover/certify | `ANM_*` | show route id, packet id, contact-target trace, artifact ref, focus target |
| traversal concrete rows in `editor/110` | `editor/76` | `route.nav.*` | `packet.nav.*` | `engine/72` | author/bind/inspect/simulate/compare/capture/recover/certify | `NAV_*` | show route id, packet id, blocked-reason trace, artifact ref, focus target |
| timeline concrete rows in `editor/110` | `editor/77` | `route.timeline.*` | `packet.timeline.*` | `engine/73` | author/bind/inspect/simulate/compare/capture/recover/certify | `TIM_*`, `CRT_*` | show route id, packet id, replay/recover trace, artifact ref, focus target |
| inventory/economy concrete rows in `editor/110` | `editor/78` | `route.inventory.*` | `packet.inventory.*` | `engine/74` | author/bind/inspect/simulate/compare/capture/recover/certify | `INV_*`, `TRD_*` | show route id, packet id, scarcity/economy trace, artifact ref, focus target |
| quest/consequence concrete rows in `editor/110` | `editor/79` | `route.quest.*` | `packet.quest.*` | `engine/75` | author/bind/inspect/simulate/compare/capture/recover/certify | `QST_*`, `EVT_*` | show route id, packet id, event/consequence trace, artifact ref, focus target |
| causality concrete rows in `editor/110` | `editor/80` | `route.why.*` | `packet.why.*` | `engine/76` | inspect/compare/capture/recover/certify | `TRC_*`, `WHY_*` | show route id, packet id, why-trace, artifact ref, focus target |
| hardware-floor concrete rows in `editor/110` | `editor/81` | `route.hardware.*` | `packet.hardware.*` | `engine/69` + `engine/98` + `engine/102` | author/bind/inspect/simulate/compare/capture/recover/certify | `FLR_*` | show route id, packet id, floor result, artifact ref, focus target |
| project / workspace / build / export / launch concrete rows in `editor/110` | `editor/82` | `route.project.*`, `route.workspace.*`, `route.build.*`, `route.export.*`, `route.launch.*` | `packet.project.*` and peers | see exact rows in `editor/110` | bootstrap/save/build/export/launch | `PRJ_*`, `BLD_*`, `EXP_*`, `LCH_*` | show route id, packet id, build/export trace, artifact ref, focus target |
| play / inspect / compare / capture / certify concrete rows in `editor/110` | `editor/86` | `route.sim.*`, `route.inspect.*`, `route.compare.*`, `route.capture.*`, `route.certify.*` | `packet.sim.*` and peers | see exact rows in `editor/110` | play/inspect/compare/capture/certify | `SIM_*`, `TRC_*`, `CRT_*` | show route id, packet id, compare/capture trace, artifact ref, focus target |
| freeze / recovery concrete rows in `editor/110` | `editor/87` | `route.freeze.*`, `route.recover.*`, `route.trace.*` | `packet.freeze.*`, `packet.recover.*`, `packet.trace.*` | see exact rows in `editor/110` | review/signoff/recover/trace | `FRZ_*`, `FLR_*`, `TRC_*` | show route id, packet id, freeze/recover trace, artifact ref, focus target |
| frame and render substrate concrete rows in `editor/110` | `editor/90`, `editor/91` | `route.frame.*`, `route.render.pipeline.*` | `packet.frame.*`, `packet.render.pipeline.*` | `engine/86` | author/bind/inspect/simulate/compare/capture/recover/certify | `deny.frame.*`, `deny.pipeline.*` | show route id, packet id, framegraph trace, artifact ref, focus target |
| material and shader concrete rows in `editor/110` | `editor/92` | `route.render.material.*` | `packet.render.material.*` | `engine/88` | author/bind/inspect/simulate/compare/capture/recover/certify | `deny.material.*` | show route id, packet id, material truth trace, artifact ref, focus target |
| residency concrete rows in `editor/110` | `editor/93` | `route.render.residency.*` | `packet.render.residency.*` | `engine/89` | author/bind/inspect/simulate/compare/capture/recover/certify | `deny.residency.*` | show route id, packet id, residency trace, artifact ref, focus target |
| lighting / sky / atmosphere concrete rows in `editor/110` | `editor/94` | `route.sky.*` | `packet.sky.*` | `engine/90` + `engine/91` | author/bind/inspect/simulate/compare/capture/recover/certify | `deny.light.*`, `deny.sky.*` | show route id, packet id, visibility trace, artifact ref, focus target |
| audio concrete rows in `editor/110` | `editor/95` | `route.audio.*` | `packet.audio.*` | `engine/93` | author/bind/inspect/simulate/compare/capture/recover/certify | `deny.audio.*` | show route id, packet id, audio trace, artifact ref, focus target |
| animation-runtime concrete rows in `editor/110` | `editor/96` | `route.anim.runtime.*` | `packet.anim.runtime.*` | `engine/96` | author/bind/inspect/simulate/compare/capture/recover/certify | `deny.animruntime.*` | show route id, packet id, contact-solve trace, artifact ref, focus target |
| VFX concrete rows in `editor/110` | `editor/97` | `route.vfx.*` | `packet.vfx.*` | `engine/94` | author/bind/inspect/simulate/compare/capture/recover/certify | `deny.vfx.*` | show route id, packet id, composite trace, artifact ref, focus target |
| UI runtime concrete rows in `editor/110` | `editor/98` | `route.ui.*` | `packet.ui.*` | `engine/95` | author/bind/inspect/simulate/compare/capture/recover/certify | `deny.ui.*` | show route id, packet id, runtime-ui trace, artifact ref, focus target |
| trace / diagnostics concrete rows in `editor/110` | `editor/100` | `route.trace.*` | `packet.trace.*` | `engine/76` + `engine/97` | inspect/capture/follow | `TRC_*` | show route id, packet id, trace lineage, artifact ref, focus target |
| certification concrete rows in `editor/110` | `editor/103` | `route.cert.*` | `packet.cert.*` | `engine/84` + `engine/97` | inspect/simulate/compare/capture/recover/certify/append | `deny.cert.*` | show route id, packet id, cert trace, artifact ref, focus target |
| evidence concrete rows in `editor/110` | `editor/105` | `route.evidence.*` | `packet.evidence.*` | `engine/84` + `engine/97` | inspect/simulate/compare/capture/recover/certify/append | `deny.evidence.*` | show route id, packet id, append trace, artifact ref, focus target |
| release-freeze concrete rows in `editor/110` | `editor/109` | `route.freeze.*` | `packet.freeze.*` | `engine/84` + `engine/97` | inspect/simulate/compare/capture/recover/review/certify/signoff | `deny.freeze.*` | show route id, packet id, freeze trace, artifact ref, focus target |

## Law
- every exact production `btn.*` id must live in `editor/110`;
- descriptive family notation is allowed here only as a summary over concrete manifest rows;
- no active production command may use legacy `button` namespace;
- editor may never skip tooling and emit sdk packets directly;
- any row participating in compare, capture, recover, certify, or freeze must return a traceable `artifact_ref` on terminal success.
