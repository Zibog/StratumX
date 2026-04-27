# Engine Truth To SDK To Tooling To Editor Diagnostics Capture Chain Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the singular upward chain from engine publications to editor overlays, compare/capture, evidence append, and recovery focus.

## Upward rows
| engine publication family | sdk observation family | tooling normalization and artifact path | editor target lab | overlay family | capture mode | compare mode | evidence append rule |
|---|---|---|---|---|---|---|---|
| `event.hyd.*` | `obs.hyd.*` | `route.trace.hyd.*` + retained ledger bundle | `editor/66`, `editor/100`, `editor/103`, `editor/105` | `overlay.hyd.*` | `capture.hyd.ledger_bundle` | `compare.hyd.restore_triplet` | append only after triplet closure |
| `event.storm.*`, `event.fire_weather.*` | `obs.storm.*` | `route.trace.storm.*` + retained visibility bundle | `editor/67`, `editor/94`, `editor/100`, `editor/103`, `editor/105` | `overlay.storm.*` | `capture.storm.visibility_bundle` | `compare.storm.front_track` | append only after threshold row and baseline are present |
| `event.soft.*`, `event.anim_synthesis.*` | `obs.soft.*` | `route.trace.soft.*` + retained contact bundle | `editor/68`, `editor/75`, `editor/100`, `editor/105` | `overlay.soft.*` | `capture.soft.contact_bundle` | `compare.soft.contact_triplet` | append only after contact solve is stable |
| `event.soc.*` | `obs.soc.*` | `route.trace.soc.*` + retained reason bundle | `editor/69`, `editor/80`, `editor/100`, `editor/105` | `overlay.soc.*` | `capture.soc.reason_bundle` | `compare.soc.reason_triplet` | append only after reason chain and failure family are materialized |
| `event.tac.*` | `obs.tac.*` | `route.trace.tac.*` + retained cover bundle | `editor/70`, `editor/80`, `editor/100`, `editor/105` | `overlay.tac.*` | `capture.tac.cover_bundle` | `compare.tac.cover_triplet` | append only after cover-state and blocker trace are materialized |
| `event.eco.*` | `obs.eco.*` | `route.trace.eco.*` + retained migration bundle | `editor/71`, `editor/80`, `editor/100`, `editor/105` | `overlay.eco.*` | `capture.eco.migration_bundle` | `compare.eco.migration_triplet` | append only after migration cause and hazard chain are visible |
| `event.wound.*` | `obs.wound.*` | `route.trace.wound.*` + retained wound bundle | `editor/72`, `editor/80`, `editor/100`, `editor/105` | `overlay.wound.*` | `capture.wound.trace_bundle` | `compare.wound.replay_triplet` | append only after anatomy, armor, and damage trace are visible |
| `event.semantic.*` | `obs.semantic.*` | `route.trace.semantic.*` + retained grounding bundle | `editor/73`, `editor/80`, `editor/100`, `editor/105` | `overlay.semantic.*` | `capture.semantic.bundle` | `compare.semantic.consequence_triplet` | append only after grounding, denial path, and consequence binding are visible |
| `event.floor.*` | `obs.floor.*` | `route.trace.floor.*` + retained floor bundle | `editor/74`, `editor/81`, `editor/87`, `editor/109` | `overlay.floor.*` | `capture.floor.bundle` | `compare.floor.pack` | append only after blocker trace and last-good baseline pointer exist |
| `event.nav.*` | `obs.nav.*` | `route.trace.nav.*` + retained traversal bundle | `editor/76`, `editor/80`, `editor/100`, `editor/105` | `overlay.nav.*` | `capture.nav.traversal_bundle` | `compare.nav.path_triplet` | append only after blocked reason and reservation state are visible |
| `event.timeline.*` | `obs.timeline.*` | `route.trace.timeline.*` + retained restore bundle | `editor/77`, `editor/100`, `editor/103`, `editor/105` | `overlay.timeline.*` | `capture.timeline.restore_bundle` | `compare.timeline.restore_triplet` | append only after failed/current/recovered triplet exists |
| `event.inventory.*`, `event.economy.*` | `obs.inventory.*` | `route.trace.inventory.*` + retained trade bundle | `editor/78`, `editor/80`, `editor/100`, `editor/105` | `overlay.inventory.*` | `capture.inventory.trade_bundle` | `compare.inventory.state_triplet` | append only after container state, economy delta, and legality result are visible |
| `event.quest.*`, `event.world_consequence.*` | `obs.quest.*` | `route.trace.quest.*` + retained consequence bundle | `editor/79`, `editor/80`, `editor/100`, `editor/105` | `overlay.quest.*` | `capture.quest.consequence_bundle` | `compare.quest.consequence_triplet` | append only after trigger, mutation, and why chain are visible |
| `event.frame.*`, `event.pipeline.*` | `obs.frame.*` | `route.trace.frame.*` + retained framegraph bundle | `editor/90`, `editor/91`, `editor/100`, `editor/103`, `editor/105` | `overlay.frame.*` | `capture.render.frame_bundle`, `capture.render.pipeline_bundle` | `compare.render.frame_triplet`, `compare.render.frame_graph` | append only after frame timing, pass ownership, and attachment legality are visible |
| `event.material.*` | `obs.material.*` | `route.trace.material.*` + retained material bundle | `editor/92`, `editor/100`, `editor/103`, `editor/105` | `overlay.material.*` | `capture.render.material_bundle` | `compare.render.material_triplet` | append only after material truth and fallback posture are visible |
| `event.residency.*` | `obs.residency.*` | `route.trace.residency.*` + retained residency bundle | `editor/93`, `editor/100`, `editor/103`, `editor/105` | `overlay.residency.*` | `capture.render.residency_bundle` | `compare.render.residency_triplet` | append only after budget pressure, mip choice, and denial evidence are visible |
| `event.light.*`, `event.sky.*`, `event.camera.*` | `obs.light.*` | `route.trace.sky.*` + retained atmosphere bundle | `editor/94`, `editor/100`, `editor/103`, `editor/105` | `overlay.sky.*` | `capture.sky.atmosphere_bundle` | `compare.atmosphere.triplet` | append only after atmosphere, lighting, exposure/post, and weather-coupled visibility are visible |
| `event.audio.*` | `obs.audio.*` | `route.trace.audio.*` + retained audio bundle | `editor/95`, `editor/100`, `editor/103`, `editor/105` | `overlay.audio.*` | `capture.audio.mix_bundle` | `compare.audio.mix_triplet` | append only after timing, occlusion, zone mix, and surface variation are visible |
| `event.anim_runtime.*` | `obs.anim_runtime.*` | `route.trace.anim_runtime.*` + retained solve bundle | `editor/96`, `editor/100`, `editor/103`, `editor/105` | `overlay.animruntime.*` | `capture.anim.runtime_bundle` | `compare.anim.runtime_triplet` | append only after intent, pose, contact solve, and skinning are visible |
| `event.vfx.*` | `obs.vfx.*` | `route.trace.vfx.*` + retained composite bundle | `editor/97`, `editor/100`, `editor/103`, `editor/105` | `overlay.vfx.*` | `capture.vfx.bundle` | `compare.vfx.triplet` | append only after composite chain and media legality are visible |
| `event.ui.*` | `obs.ui.*` | `route.trace.ui.*` + retained runtime-ui bundle | `editor/98`, `editor/100`, `editor/103`, `editor/105` | `overlay.ui.*` | `capture.ui.layout_bundle` | `compare.ui.layout_triplet` | append only after runtime composition and text/debug legality are visible |
| `event.trace.*`, `event.cert.*`, `event.evidence.*`, `event.freeze.*` | `obs.trace.*` | `route.trace.*` + retained trace/cert/freeze bundle | `editor/100`, `editor/103`, `editor/105`, `editor/109` | `overlay.trace.*` | `capture.trace.bundle` | `compare.trace.reason` | append only after terminal result, next action, and focus target exist |

## Command monitor surfaces
The upward chain must feed these editor-visible surfaces:
- `editor/100` Command Runtime Monitor;
- `editor/100` Route/Packet Inspector;
- `editor/100` Lifecycle Timeline;
- `editor/100` Focus/Recovery Viewer;
- `editor/100` Artifact/Evidence Link Panel.

## Upward law
- no capture without `artifact_ref`;
- no compare without baseline;
- no recovery without failed-run pointer or recovery anchor;
- no evidence append before trace, compare digest, and next legal action are visible;
- every retryable failure must surface one explicit next legal action and one explicit focus target;
- no engine publication may bypass sdk observation normalization on its way to the editor.
