# Fur Hair Coverage And Promoted Local Strand Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the canonical cheap-runtime law for fur, hair, and similar dense repeated surface coverage.

This document exists so the archive can say "looks like hair/fur" without requiring strand-authoritative simulation everywhere.

## Core law
Fur and hair are canonical coverage systems.
They are authored through material packages and coverage profiles.
They are not required to exist as always-live strand simulations.

## Exact truth objects
- `CoverageMaskState`
- `CoverageDirectionField`
- `CoverageClumpField`
- `CoverageWindState`
- `CoverageWetCharDirtState`
- `CoveragePromotionState`
- `CoverageRecoveryAnchor`

## Representation ladder
The runtime must support at minimum the following richness ladder:
- `repr.summary_far` — far summary, shading/fuzz/silhouette contribution only;
- `repr.shell_or_strip_mid` — shell/strip/card-like clustered coverage;
- `repr.coverage_patch_near` — near-camera patch richness with local motion and breakup;
- `repr.promoted_local_strand_like` — promoted local strand-like richness for rare close contexts.

The promoted local mode is legal.
It is optional, tightly budgeted, and never baseline law.

## Host and territory law
Coverage systems may be attached to:
- terrain and vegetation packages;
- prop packages when declared;
- living hosts through engine `105`.

Coverage state must remain attached to the owning package or host.
It may not become detached hidden truth.

## Exact phase order
1. resolve owning package / host and active coverage profile;
2. sample legal wind, wetness, char, and dirt modifiers for the active tier;
3. update coverage motion and state summary;
4. choose richness representation;
5. publish diagnostics and recovery anchor.

## Cheapness law
The runtime may reduce:
- strand-like richness to clustered shell/strip richness;
- local patch density;
- secondary self-shadow richness;
- far-distance motion detail.

It may not:
- remove declared coverage presence without publication;
- silently change coverage ownership;
- require promoted local mode for baseline correctness.

## Resource envelope
- CPU: green <= `0.9 ms`, yellow <= `1.4 ms`, orange <= `2.0 ms`, red > `2.0 ms`;
- GPU: green <= `1.6 ms`, yellow <= `2.4 ms`, orange <= `3.2 ms`, red > `3.2 ms`;
- RAM: orange > `224 MiB`, red > `320 MiB`;
- disk / IO: none on hot path.

## Coupling boundaries
| Boundary role | Declared links | Forbidden shortcut |
|---|---|---|
| reads | root `84`, root `96`, engine `63`, engine `91`, engine `103–105` | never owns living wound verdicts or static structure collapse truth |
| publishes | editor `68`, editor `113`, root diagnostics / capture routes | may not bypass declared coverage profiles with one-off groom-only logic |

## Certification duties
- retain representation ladder trace for baseline, failed, and recovered runs;
- publish whether promoted-local mode was active;
- publish first degrade step that reduced richness;
- prove that fallback to shell/strip or summary modes preserved declared ownership and state publication.

## Current posture
`document_gold / closes_fur_hair_coverage_gap`
