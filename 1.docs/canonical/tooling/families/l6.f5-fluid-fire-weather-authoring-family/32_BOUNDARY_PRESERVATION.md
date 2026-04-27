# Boundary Preservation

This contract belongs specifically to the l6.f5 fluid fire weather authoring family family and describes family-only coordination.


## Must remain outside `fluid_fire_weather_authoring_family` ownership
- member-internal mutable authority rows for fluid, fire, weather authoring, field rules, and weather cell authoring, authority-facing minimal truth: fluid/fire/weather edit intents, snapshot classes: fluid/fire/weather snapshots, index classes: field lookup indices
- unrelated domain truth not declared in this family
- editor widget/layout state

## Operational note
This file remains active and package-specific for `l6.f5-fluid-fire-weather-authoring-family` / `32_BOUNDARY_PRESERVATION.md`.

## Scope note
The authority, dependency, and audit meaning of 32 BOUNDARY PRESERVATION is defined by the surrounding package and may not be inferred from another file with a similar filename.

## Review use
A reviewer should be able to use this document as part of an implementation checklist, not as decorative filler.
