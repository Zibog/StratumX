# Advanced Wounding Dismemberment And Species Specific Lethality Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define runtime truth for wound regions, lethality profiles, bleed and shock state, dismemberment legality, and replay recovery.

## Exact truth objects
| Truth object | Meaning |
|---|---|
| `AnatomicalRegionLedger` | region ids, damage class, armor class, and exposure |
| `BleedShockState` | bleed rate, pain/shock tier, and stabilization posture |
| `SpeciesLethalityProfile` | species-specific lethality and dismemberment thresholds |
| `DismemberRuleSet` | allowed sever classes and hard denials |
| `BallisticTraceLinkSet` | trace-to-region linkage for replay and certification |
| `WoundRecoveryAnchor` | last-good wound baseline and replay anchor |
| `ExposureTraversalDigest` | digest handed in from engine `105` for ordered layer traversal and exposure |

## Exact state machine
`intact -> injured -> bleeding / shocked -> severed / stabilized -> recovered -> certified`

## Exact phase order
1. bind incoming trace or exposure handoff
2. resolve region damage
3. apply bleed/shock
4. validate dismemberment legality
5. publish replay slice and recovery anchor

## Exact coupling boundaries
- may read ballistic trace truth only through stable links
- may read living-layer exposure only through engine `105` handoff
- may not own external corpse persistence or loot logic
- must publish one first blocking code for every denied sever or replay gap

## Exact fail / denial families
- `wound.region.link_missing`
- `wound.replay_gap`
- `wound.dismember.denied`
- `wound.species.profile_missing`

## Exact resource envelope
cpu `<= 3.6 ms`; gpu overlays `<= 0.5 ms`; ram `<= 256 MiB`; disk/IO certification artifacts only

## Exact replay window
stable 10-second wound replay with checkpoints at hit, escalation, stabilize, sever, and recovery

## Exact certification duties
- retain wound baseline, failed/recovery replay triplet, species profile ref, first blocker board
- retain trace link snapshot
- retain recovery anchor id
- retain incoming exposure traversal digest when layer traversal was involved

## Exact legal recovery actions
- restore wound baseline
- drop one replay horizon tier only if compare law allows
- rerun wound replay compare

## Exact domain command families
- `verify.wound_trace`
- `compare.wound_triplet`
- `capture.region_trace`
- `recover.wound_anchor`

## Current posture
`document_gold / doc_closed_impl_open`
