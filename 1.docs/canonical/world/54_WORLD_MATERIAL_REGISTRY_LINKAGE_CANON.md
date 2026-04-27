# World Material Registry Linkage Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze how a world package links terrain layers, local instances, surface families, response profiles, and overlays into the material-first contour.

## Linkage law
A world package must be able to answer, for every world-authored surface region:
- which surface family is bound;
- which material archetype is bound;
- which response profile is bound;
- which local material instance stack, if any, overrides shared defaults;
- which biome or aftermath overlay participates.

## Unified surface-family law
Terrain surface families, prop surface families, and structural surface families may differ in content, but they may not diverge into incompatible linkage vocabulary.
World-level material linkage must stay readable against root `84–87` and root `88`.

## Deny conditions
- unresolved surface-family ref;
- unresolved response-profile ref;
- local instance that breaks canonical registry linkage;
- overlay binding that has no declared material consequence meaning.


## Expanded unified surface-family law
The unified language is mandatory across:
- terrain surface family;
- prop surface family;
- structure surface family;
- foliage surface family.

Differences in content are legal.
Differences in ownership vocabulary are not.

## Implementation-tail linkage law
When world authoring binds or mutates any surface-family linkage, the world layer must declare:
- authoritative linkage owner;
- chunk, placement, or object scope affected;
- invalidated caches and indices;
- pending-save posture;
- validation blockers for later world validation;
- recovery anchor for return to last-good linkage state.

## Normalized branch linkage
World material linkage is incomplete unless a linked instance can resolve:
- physical response family coverage
- visual response family coverage
- acoustic response family coverage
- light response family coverage
- cheap-runtime posture coverage
through the canonical vocabulary in root `89–93`.
