# Heavy Domain Authoring Transaction And Undo Constitution Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze one transaction law for destructive, hydrologic, climatic, social, tactical, biological, and grooming authoring so every heavy-domain mutation travels through one intelligible and reversible conveyor.

## Governing law
A heavy-domain authoring action is legal only when it carries:
- one stable request id;
- one authority owner;
- one target identity set;
- one precondition set;
- one validation set;
- one failure family;
- one undo posture;
- one replay/cert posture;
- one focus-recovery target;
- one invalidation publication contract.

## Mandatory transaction rows

| Family | Primary intent | Primary targets | Companion sdk family | Companion tooling family | Primary editor surfaces |
|---|---|---|---|---|---|
| terrain deformation authoring | carve / deposit / smooth / stitch | terrain ref, deformation scope, dirty volume | sdk `81` `packet.tx.terrain_deformation_author.v1` | tooling `85–86` `route.terrain.*` | editor `119`, `127` |
| destruction topology authoring | support groups, breach masks, collapse legality | topology refs, support groups, structural class | sdk `81` `packet.tx.destruction_topology_author.v1` | tooling `85–86` `route.struct.*` | editor `120`, `127` |
| weather front injection | front seeds, region scope, arrival window | front refs, region scope, timeline anchors | sdk `81` `packet.tx.weather_front_inject.v1` | tooling `85–86` `route.climate.*` | editor `136`, `127` |
| hydrology source edit | inventory, leak geometry, contamination, overflow law | container/source refs, fluid inventory refs | sdk `81` `packet.tx.hydrology_source_edit.v1` | tooling `85–86` `route.hyd.*` | editor `121`, `127` |
| tactical sandbox edit | shared intent, reservations, cover rules | squad refs, cover scopes, reservation refs | sdk `81` `packet.tx.tactical_sandbox_edit.v1` | tooling `85–86` `route.tac.*` | editor `137` |
| species wound rule edit | body topology classes, lethality matrix, gore legality | species topology refs, wound rule bundles | sdk `81` `packet.tx.species_wound_rule_edit.v1` | tooling `85–86` `route.wound.*` | editor `138` |
| groom setup edit | coverage masks, rung, wind/wetness response | groom asset refs, coverage masks, rung ids | sdk `81` `packet.tx.groom_setup_edit.v1` | tooling `85–86` `route.groom.*` | editor `125` |

## Transaction state law

| Stage | Required outcome |
|---|---|
| request accepted | request id, authority owner, and target ids are frozen |
| preconditions checked | legal target and scope posture are resolved |
| validation executed | domain-specific gates and blocker ids are produced |
| apply attempted | authoritative mutation or explicit deny is emitted |
| invalidation published | downstream caches, captures, and comparisons are marked dirty |
| artifact posture resolved | saved bundle, temp preview, or no-artifact posture is declared |
| focus recovery emitted | target surface, blocker pane, or last-good anchor is declared |
| commit sealed | retained event/result family is written for replay/cert if the family requires it |

## Transaction payload law

| Payload row | Requirement |
|---|---|
| stable target identity | targets must be stable refs, never view-local indices |
| scope declaration | world/region/cell/object scope must be explicit |
| undo anchor | reversible transactions must snapshot or reference last-good anchor before apply |
| compare identity | replay and compare ids must be derivable from the transaction itself |
| blocker family | every deny path must produce one declared blocker family |
| route family | transaction must name its operator and tooling route family |
| narrow proof posture | proof-lane transactions must declare whether they are slice-local or full-runtime affecting |

## Failure ladder

| Failure family | Meaning |
|---|---|
| `TX_PRECOND_*` | missing target, illegal scope, missing authority, or stale ref |
| `TX_VALID_*` | domain validation failed before apply |
| `TX_APPLY_*` | authoritative mutation failed or partially applied |
| `TX_UNDO_*` | undo anchor missing or unsafe |
| `TX_SAVE_*` | artifact persistence or bundle sealing failed |
| `TX_FOCUS_*` | route could not land on the required recovery surface |
| `TX_INVALIDATE_*` | downstream dirty publication could not be emitted lawfully |

## Undo / redo law
- exact truth mutations require reversible anchors or an explicit non-reversible certification posture;
- redo may not synthesize side effects absent from the original transaction;
- if safe undo is impossible, the operator must receive a freeze-grade warning before commit;
- recoverable preview steps may be transient, but committed heavy-domain authoring may not hide its last-good anchor;
- proof-slice transactions may be narrower than world-authoring transactions but may not lie about their retained scope.

## Disabled-reason law
A disabled heavy-domain control must publish:
- the missing precondition;
- the blocking family code;
- the target required to unblock;
- the next legal action surface;
- whether the block is local, scope-wide, or certification-wide.

## Companion execution surfaces
- sdk `81` and `87` define transaction packet families and failure/result carriers;
- tooling `85–86` define stage sequencing, blocker recovery, and focus return;
- editor `119–127`, `132–138` define controls and disabled reasons;
- root `106`, `116`, and `117` must expose the same family names and first-playable constraints.

## Acceptance obligations
A heavy-domain transaction family is not canon-closed until:
- sdk `81` declares the request/result families;
- tooling `85–86` declares stages, blockers, and recovery surfaces;
- editor `119–127` or `132–138` exposes human-visible controls and disabled reasons;
- root `76` includes at least one signal row using that transaction family;
- replay/capture obligations are cross-linked into root `107` and sdk `82/88` when retained proof is required.

## Current posture
`document_gold / transaction_constitution_closed / runtime_impl_open`
