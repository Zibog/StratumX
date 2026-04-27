# Dream Stack Asset Canonicalization Law Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze one canonicalization law for the heavy asset families required by the dream-stack so import, cook, compare, and replay all operate on stable asset identities.

## Asset family law

| Family | Canonical target package | Mandatory canonical rows | Primary companions |
|---|---|---|---|
| terrain heightfields | canonical terrain-height package | resolution, sample encoding, world scale, source lineage | tooling `83/89`, editor `119/123` |
| terrain material textures | canonical terrain-material package | texture refs, uv scale, tint, cross-section family, mip policy | tooling `83/89`, editor `119/122` |
| destruction masks and topology atlases | canonical destruction-asset package | mask family ids, support graph refs, cross-section atlas refs, topology lineage | tooling `83/89`, editor `120` |
| hydrology source presets | canonical fluid-source package | inventory law, contamination classes, overflow posture, freeze classes | tooling `83/89`, editor `121` |
| climate and weather presets | canonical climate package | front classes, cloud families, precipitation law, baseline timeline anchors | tooling `83/89`, editor `136` |
| photoreal material and lighting assets | canonical photoreal package | shading family, streaming policy, fallback tiers, scene-class tags | tooling `83/84/89`, editor `126/135` |
| audio banks and surface response packs | canonical audio package | bank ids, surface response ids, routing class, memory/streaming posture | tooling `83/84/89`, editor `62/135` |
| fur/hair/coverage assets | canonical microgeometry package | coverage masks, rung class, wet/char/wind variants, memory posture | tooling `83/84/89`, editor `125` |
| motion priors and contact assets | canonical motion package | pose priors, style class, contact constraints, species/body class | tooling `83/84/89`, editor `134` |

## Canonicalization rules
- every heavy asset family must retain source lineage;
- every canonical asset must be environment-independent and compare-friendly;
- asset names may not be the only identity; canonical package ids are required;
- family-specific normalization must be explicit and replay-safe;
- first-playable proof assets may be narrow subsets but may not use ad-hoc uncatalogued formats.

## Validation law

| Validation family | Meaning |
|---|---|
| `ASSET_SRC_*` | source asset invalid or unreadable |
| `ASSET_FAMILY_*` | family-specific normalization failed |
| `ASSET_SCHEMA_*` | canonical package malformed |
| `ASSET_LINEAGE_*` | source lineage missing or broken |
| `ASSET_COOK_*` | canonical asset cannot enter runtime cook legally |
| `ASSET_COMPARE_*` | package cannot participate in benchmark or golden diff lawfully |

## Companion execution surfaces
- tooling `83` is the authoritative import and canonicalization route;
- tooling `89` catalogs asset family laws and blockers;
- sdk `79–84` must name the packet families that expose these assets downstream;
- editor `107`, `119`, `120`, `121`, `122`, `125`, `126`, `134`, and `136` are the primary operator surfaces;
- root `109`, `116`, and `117` must describe the same canonical asset families.

## Acceptance obligations
An asset family is not canon-closed until:
- one canonical package class exists;
- one normalization and blocker family is explicit;
- one operator surface exists for assign or inspect;
- one tooling route exists for import/canonicalize/cook;
- one compare posture exists when the family affects retained proof.

## Current posture
`document_gold / asset_canonicalization_closed / runtime_asset_build_open`
