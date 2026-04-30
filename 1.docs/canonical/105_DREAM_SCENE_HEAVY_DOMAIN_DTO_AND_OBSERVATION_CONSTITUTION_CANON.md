# Dream Scene Heavy Domain DTO And Observation Constitution Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze the public-observation and public-packet shape for heavy dream-stack domains so editor, tooling, and certification never fall back to ad-hoc structs or direct engine leakage.
This document is the root constitution for heavy DTO families.
Sdk packet tables, tooling routes, and editor labs are the companion execution surfaces.

## Governing law
A heavy domain does not become operator-visible merely because engine truth exists.
It becomes visible only when all of the following are declared and cross-linked:
- one public DTO family;
- one observation class and cadence posture;
- one retention posture;
- one consumer table;
- one failure family;
- one compare/capture posture;
- one version and compatibility posture;
- one downgrade or absence law.

## Mandatory DTO families

| Family | Public concern | Primary sdk family | Primary consumers | Required observation classes |
|---|---|---|---|---|
| destruction topology | support graph, breach class, collapse eligibility, fragment release | sdk `79` `packet.destruction.topology_delta.v1` | editor `120`, tooling `50/85/86` | event-driven, snapshot-only, certification-only |
| terrain deformation | crater/gouge/trench/churn and consequences | sdk `79` `packet.terrain.deformation_slice.v1` | editor `119`, tooling `43/85/86` | live-per-frame, event-driven, certification-only |
| hydrology state | inventory, leak geometry, inflow/outflow, contamination, freeze | sdk `79` `packet.hydrology.container_state.v1` | editor `121`, tooling `48/55/85/86` | live-per-frame, event-driven, snapshot-only, certification-only |
| climate theater | fronts, cells, precipitation, lightning, lunar coupling | sdk `79` `packet.climate.front_state.v1` | editor `136`, tooling `48/84/85/86` | live-per-frame, event-driven, certification-only |
| squad tactics | shared intent, role assignment, suppression, flank reservation, cover validity | sdk `79` `packet.tactics.squad_intent.v1` | editor `137`, tooling `49/85/86` | live-per-frame, event-driven, certification-only |
| society deltas | needs, resources, status, crime, reputation | sdk `79` `packet.society.delta.v1` | editor `137`, tooling `49/57/85/86` | event-driven, snapshot-only, certification-only |
| layered wounds | traversal layers, organ zones, bone hit, survivability, gore legality | sdk `79` `packet.wound.layered_result.v1` | editor `138`, tooling `50/55/85/86` | event-driven, snapshot-only, certification-only |
| microgeometry coverage | fur/hair/foliage representation rung, wind/wetness/char state, budget posture | sdk `79` `packet.microgeometry.coverage_state.v1` | editor `125`, tooling `48/53/83/85/86` | live-per-frame, snapshot-only, certification-only |
| photoreal fallback | hardware profile, lighting/shadow/volumetric/reflection rung, first blocker | sdk `79` `packet.photoreal.fallback_report.v1` | editor `126`, tooling `52/59/84/86` | event-driven, snapshot-only, certification-only |

## Required DTO shape rows

| Row | Requirement |
|---|---|
| stable identity | packet carries stable packet id, stable subject id, and subject family |
| scope identity | packet names world scope, region scope, cell scope, or local object scope explicitly |
| temporal posture | packet names timestamp or simulation tick and whether it is frame, event, snapshot, or retained artifact |
| cadence class | packet names the legal subscription class and whether drop/coalesce is allowed |
| authoring provenance | packet names the producing owner family and route family |
| degradation posture | packet names whether it is exact, reduced-exact, statistical, retained-summary, or fallback-only |
| compare identity | packet carries compare-friendly identity rows so capture/replay can diff without ad-hoc field guessing |
| failure attachment | packet family defines what failure family is emitted when production or delivery fails |

## Observation classes

| Class | Contract | Delivery posture | Retention posture | Allowed consumers |
|---|---|---|---|---|
| live-per-frame | bounded, view-scoped, drop-safe only under declared degrade | subscribed stream | transient unless captured | runtime labs, focused inspectors |
| event-driven | exact retained event with stable id and cause lineage | event bus / retained ledger | retained by policy | labs, reason tools, replay tools |
| snapshot-only | full digest or heavy slice at operator request or checkpoint | explicit query / checkpoint emit | retained by artifact policy | compare, audit, certification |
| debug-only | lab-only explanatory payload that may be expensive | on demand | non-freeze unless promoted | diagnostics and why surfaces only |
| certification-only | immutable retained artifact class | explicit run-step emission | retained and sealed | cert, freeze, audit, golden diff |

## Compatibility posture

| Compatibility row | Required law |
|---|---|
| additive field introduction | must be optional until sdk compatibility tables declare required status |
| field removal or rename | must mint a new packet version |
| cadence change | must be treated as a compatibility event and update sdk `80/86` |
| scope widening | must update editor entitlement and tooling routing before use |
| consumer entitlement | no consumer may silently parse a family it is not listed against |

## Failure ladder

| Failure family | Meaning |
|---|---|
| `DTO_SCHEMA_*` | required field family absent or malformed |
| `OBS_SCOPE_*` | packet was requested outside its legal scope/window |
| `OBS_RATE_*` | cadence exceeds allowed subscription budget |
| `OBS_RETAIN_*` | retention class missing or illegally downgraded |
| `DTO_OWN_*` | packet was emitted by a non-authoritative owner |
| `DTO_CONSUMER_*` | consumer tried to use a family outside declared entitlement |
| `DTO_VER_*` | producer and consumer disagreed on packet family version or downgrade posture |

## Consumer law
- editor may consume only declared sdk packet families;
- tooling may not invent packet family names ad hoc;
- engine-private structs may not be mounted directly into editor or tooling;
- any new heavy domain must declare cadence, retention, compatibility, and consumers before it becomes operator-visible;
- first-playable proof lanes may consume only the narrow retained families required for the proof slice and may not silently widen into full-dream subscriptions.

## Companion execution surfaces
- sdk `79–80`, `83–86` define packet families, cadence, retention, and catalogs;
- tooling `83–86`, `89–92` define routing, invalidation, compare, and recovery;
- editor `119–127`, `132–138` define operator controls, overlays, and captures;
- root `76`, `107`, `115`, and `117` must reflect the same family names.

## Acceptance obligations
A heavy-domain DTO family is not canon-closed until:
- its public packet appears in sdk `79–82`;
- its subscriptions and cadence appear in sdk `80`;
- its route families appear in tooling `83–86`;
- its operator controls appear in editor `119–127` or `132–138`;
- its signal row appears in root `76`;
- its compare/replay family appears in root `107` and sdk `82/88`.

## Current posture
`document_gold / dto_constitution_closed / implementation_open`
