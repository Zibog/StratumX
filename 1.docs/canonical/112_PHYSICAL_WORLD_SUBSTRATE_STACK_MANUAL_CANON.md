# Physical World Substrate Stack Manual Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Provide the stitched reading spine for the physical substrate stack so material pairs, terrain deformation, fragments, hydrology, fire/wetness/smoke/wind, and aftermath all read as one lawful runtime system.

## Scope of this manual
This manual stitches:
- root `84–98`, `104`, `112`;
- engine `109–113`, `116–119`, `133–136`;
- sdk `79–82`, `85–88`;
- tooling `83–87`, `89–92`;
- editor `119–123`, `129–133`, `138` where layered traversal matters.

## Reading order
1. root material-first band `84–98`;
2. root `104` for substrate and conflict law;
3. engine `109–113` for fields, hydrology, fire numerics, climate coupling;
4. engine `116–119`, `133–136` for contact matrix, fragments, projectile and wound traversal;
5. editor `119–123`, `133`, `138` for operator lanes;
6. tooling and sdk companion routes for transaction, compare, and certification.

## Mandatory exact outcomes
- every material pair, field, deformation, or fragment event has a legal runtime order;
- save/reopen and replay law stay aligned with the same substrate families;
- physical aftermath is distinguishable from temporary visual effect;
- first-playable proof uses the same substrate law on a narrow scene, never a fake shortcut.

## Companion closure checklist
Implementers must be able to identify from companion docs:
- authoritative owner;
- public packet family;
- authoring route;
- operator surface;
- compare/replay artifact;
- blocker and fallback families.

## Current posture
`document_gold / stitched_manual_closed`
