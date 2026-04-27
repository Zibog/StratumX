# Heavy Domain Packet Family And Schema Constitution Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the non-negotiable packet families for heavy-domain simulation and authoring.

## Mandatory packet rows

| Family | Packet prefix | Stable id law | Required owner | Required consumers |
|---|---|---|---|---|
| world-scale geodesy | `packet.world.*` | region, cell, ring, and world-frame ids are stable across save/restore | engine world truth | editor `132`, tooling compare/replay |
| field substrate | `packet.field.*` | cell, surface, object, and volume ids are explicit | engine substrate truth | editor `133`, tooling certification |
| deformation / topology | `packet.deform.*`, `packet.topology.*` | mutation ids and affected scope ids are stable | engine destruction/deformation truth | editor `120`, tooling diff routes |
| hydrology | `packet.hydrology.*` | container, source, and mass-ledger ids are stable | engine hydrology truth | editor `121`, tooling replay/certification |
| living state | `packet.living.*` | actor, squad, species, and tick scope ids are stable | engine living truth | editor `134`, `137`, tooling replay |
| projectile / wound | `packet.projectile.*`, `packet.wound.*` | projectile, host, and traversal ids are explicit | engine combat truth | editor `138`, tooling evidence |
| photoreal fallback | `packet.photoreal.*` | scene, run, and hardware-profile ids are stable | engine render truth | editor `126`, `135`, tooling golden diff |
| proof-lane progress | `packet.first_playable.*` | project, world, and proof-lane ids are explicit | mixed stack handoff surfaces | editor `123`, `124`, tooling `87–88` |

## Field-table law
Every public heavy-domain packet must expose:
- packet version;
- packet family id;
- producer package and owner subsystem;
- required stable ids;
- declared optional fields;
- invalidation trigger;
- compatibility break trigger;
- primary consumer list.

## Compatibility law
Breaking field or semantic changes require a packet version bump.
Semantic reinterpretation without version change is forbidden.

## Failure law
A packet family is invalid if it omits stable ids, silently changes field meaning, or forces a consumer to infer state from presentation-only data.
