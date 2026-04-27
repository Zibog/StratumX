# Material Pair Contact Matrix And Override Precedence Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Close the legality gap between material-first authoring and actual runtime contact resolution.

## Pair resolution key
`material_a x material_b x event_type x state_modifiers -> contact verdict row`

## Event types
- footstep / locomotion
- projectile impact
- blade/contact cut
- rigid-body scrape
- cloth contact
- wind-loaded cloth/fur interaction
- body tissue traversal

## Canonical outputs
- friction class
- restitution class
- stick/slip/scrape semantic
- debris / fragment / sound / visual trigger class
- traversal consequence class

## Override precedence
`archetype -> surface_family -> response_profile -> state_modifier -> fallback`

## Fallback law
Fallback is legal only when it publishes its fallback row id and does not suppress the missing-pair blocker during certification.

## Failure families
- `material.pair_row_missing`
- `material.override_precedence_break`
- `material.fallback_hidden`
- `material.contact_output_incomplete`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
