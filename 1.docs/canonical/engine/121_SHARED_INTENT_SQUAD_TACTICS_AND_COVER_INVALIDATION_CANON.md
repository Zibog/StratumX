# Shared Intent Squad Tactics And Cover Invalidation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own group tactics as shared-intent runtime truth rather than coincidental parallel AI.

## Exact truth objects

| Object | Role |
|---|---|
| `SquadIntentPacket` | squad-level goal, time horizon, and engagement posture |
| `RoleAssignmentLedger` | suppressor, flanker, breacher, reserve, medic, scout, and fallback role allocation |
| `SuppressionAccountingLedger` | current suppression pressure and spend against target scopes |
| `FlankReservationSet` | reserved path, arrival window, and conflict state |
| `CoverValidityDigest` | whether cover remains tactically valid after world changes |
| `RetreatRegroupVerdict` | legal retreat or regroup trigger and destination |
| `CohesionStateLedger` | morale, command continuity, comms health, and spacing integrity |

## Update order
`intent publish -> role assignment -> path / flank reservation -> suppression spend -> cover validity refresh -> retreat/regroup eval -> downstream publications`

## Hard law
- squad members act from shared packets plus local capability constraints;
- destruction may invalidate cover and force plan recompute;
- ammo, fear, pain, comms, and line-of-sight may modulate but not silently bypass shared-intent law.

## Mandatory publications
- `packet.living.squad_intent.v1`
- `packet.living.cover_validity.v1`
- `packet.living.tactic_result.v1`
- explanation anchors for engine `125` and editor `137`

## Failure families
- `tactics.intent_packet_missing`
- `tactics.flank_reservation_conflict`
- `tactics.cover_invalidation_gap`
- `tactics.cohesion_break_unpublished`
- `tactics.retreat_verdict_missing`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
