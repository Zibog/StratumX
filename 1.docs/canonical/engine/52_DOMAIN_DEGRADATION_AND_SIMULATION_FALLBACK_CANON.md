# Domain Degradation and Simulation Fallback Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the law for honest fallback when simulation breadth exceeds current budgets.
The stack is allowed to degrade.
It is not allowed to lie.

## Degradation classes
A domain may degrade only through declared classes such as:
- lower spatial resolution;
- lower temporal resolution;
- reduced neighbor fan-out;
- simplified solver path;
- statistical approximation;
- deferred cold-region evaluation.

## Global fallback law
A degradation is legal only when:
- the domain still produces runtime-owned truth;
- diagnostics can identify the active fallback class;
- higher layers do not present the result as full-fidelity truth;
- irreversible world mutations remain legal under the chosen approximation.

## Domain-specific anchors
| Domain | Legal degrade examples | Illegal degrade examples |
|---|---|---|
| combat | simplified ricochet or penetration detail under budget pressure | fake death state from editor or tooling without runtime-owned consequence |
| fire/weather | coarser field resolution or reduced spread fan-out | decorative fire/weather with no runtime ownership |
| destruction | chunked aftermath evaluation or delayed debris settle | showing intact cover while runtime collision already changed, or vice versa |
| world/runtime | cold-region statistical updates | silently dropping authoritative changes |

## Publication law
Every degraded route must be able to publish:
- degraded class;
- affected domain;
- operator-visible consequence;
- whether the result is still safe for save/export/build decisions.
