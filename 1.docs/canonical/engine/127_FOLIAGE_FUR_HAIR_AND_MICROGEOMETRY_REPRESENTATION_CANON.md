# Foliage Fur Hair And Microgeometry Representation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own representation ladders for foliage, fur, and hair without fake “one shader fits all” shortcuts.

## Representation ladder

| Rung | Intended distance / pressure posture | Required guarantees |
|---|---|---|
| `promoted_local_strands` | near hero subjects | strand motion, wetness, char, and silhouette exactness |
| `groom_cards_hybrid` | near/mid compromise | readable directional mass plus limited strand detail |
| `cards_shells` | mid distance or pressure | stable silhouette and shadow family participation |
| `coverage_only` | far or strong pressure | density and directional cover remain truthful |
| `far_silhouette_summary` | very far retained view | world readability and observability only |

## State modifiers
Wetness, char, dirt, and wind must modulate the chosen representation rung, not bypass it.
Near/mid/far transitions must publish one representation-rung code and one first-downgrade reason.

## Budget law
Overdraw, memory, and shadow cost are first-class blockers.
A legal fallback must preserve silhouette family and material family, even when strand detail is reduced.

## Failure families
- `microgeom.rung_hidden`
- `microgeom.wind_response_gap`
- `microgeom.silhouette_break`
- `microgeom.overdraw_red`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
