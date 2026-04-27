# Composition

## Member composition
- terrain layers, deformation intents, brush results, and terrain manifests
- authority-facing minimal truth: terrain edit intents only
- snapshot classes: terrain snapshots
- index classes: terrain indices
- derived classes: terrain-derived overlays

## Composition rule
The family exists to make domain adjacency for `terrain_deformation_authoring_family` explicit, not to merge members into one truth owner.
