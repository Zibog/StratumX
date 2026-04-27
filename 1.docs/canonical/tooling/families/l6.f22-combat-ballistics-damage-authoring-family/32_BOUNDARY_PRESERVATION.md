# Boundary Preservation

This contract belongs specifically to the l6.f7 combat ballistics damage authoring family and describes family-only coordination.


## Must remain outside `combat_ballistics_damage_authoring_family` ownership
- member-internal mutable authority rows for combat, ballistics, and damage authoring graphs and tuning rules, authority-facing minimal truth: combat/ballistics edit intents, snapshot classes: combat snapshots, index classes: combat lookup indices
- unrelated domain truth not declared in this family
- editor widget/layout state

## Operational note
This file remains active and package-specific for `l6.f22-combat-ballistics-damage-authoring-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
