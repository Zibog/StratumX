# Asset/Model Pipeline, Content Browser, Import, Binding, and Cook Lab Canon

Stack version: `SX-CANON/1.0.27/STACK-v33`

Role: **editor operator lab**.


## Lab surfaces

| Surface | Controls |
|---|---|
| Import tray | source path, family, template, unit/axis policy, import profile. |
| Validation panel | units, scale, axes, materials, skeleton, collision, texture channels, missing data. |
| Preview viewport | mesh/material/collision/LOD/proxy preview. |
| Runtime binding panel | material slots, collision handle, LOD chain, residency class, fallback. |
| Cook panel | package identity, platform profile, blockers, warnings, evidence. |

## Required disabled reasons

- no source selected;
- unsupported format;
- unit normalization failed;
- missing material slot;
- skeleton incompatible;
- collision generation failed;
- no runtime binding;
- cook profile missing.

## Non-negotiable rules

| Rule | Meaning |
|---|---|
| No fake success | A route may return `NotImplemented`, `Blocked`, `Unavailable`, or `Unsupported`, but it may not return success for an unimplemented behavior. |
| One owner | Every truth object has exactly one owner layer. Other layers may hold handles, DTOs, views, or cached projections only. |
| Observable failure | Every failure family must produce an error code, disabled reason, recovery hint, and retained diagnostic packet. |
| Editor honesty | The editor may expose the route, preview, capture, or recovery action, but it must not mutate engine truth except through lawful SDK/tooling ingress. |
| Evidence or it did not happen | Release-grade claims require capture, compare, replay, or retained diagnostic evidence. |

## Required completion shape

Every implementation derived from this document must include:

1. owner module or crate;
2. public data contracts;
3. lifecycle stages;
4. failure and disabled reason codes;
5. editor/tooling/SDK contact points;
6. quality tests and negative-path tests;
7. retained evidence artifacts for certification routes.
