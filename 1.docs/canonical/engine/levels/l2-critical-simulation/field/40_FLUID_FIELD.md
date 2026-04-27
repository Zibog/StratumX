# Fluid Field

## Role

Bounded fluid-field solve.

## Field Matrix

| Concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| regional solve scope | bounded regional tiles only | approximate far tiles first | global fluid pass every tick |
| cross-family coupling | bounded bridge records only | defer excess coupling first | direct ownership grab |
| border consistency | obey region-border consistency law | clamp to canonical border rule | hidden border widening |

## Rule

Fluid field may solve only inside bounded regional tiles and bounded bridge classes. It may not widen solve scope through hidden cross-chunk spill or cross-family ownership.

## Field Class Matrix

| Field class | Canonical posture | Illegal posture |
|---|---|---|
| near regional tiles | preserve first | far field steals near reserve |
| far decorative flow | degrade first | full-world solve every tick |
| cross-family coupling | shaped bridge only | direct ownership grab |

## Local Operating Law

Fluid field stays region-bounded, bridge-bounded, and apply-bounded.
It may not normalize global passes or border-rule drift.
