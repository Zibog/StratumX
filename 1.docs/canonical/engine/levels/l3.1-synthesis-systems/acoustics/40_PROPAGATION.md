# Propagation

## Role

Bounded acoustic propagation solve.

## Propagation Matrix

| Concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| solve cadence | bounded acoustics cadence only | reduce ambience/detail first | frame-rate full solve for all emitters |
| probe usage | bounded probe tiers only | drop low-value probes first | unlimited probe growth |
| reflection inputs | bounded material/geometry reads only | approximate before breach | acoustics-owned world query expansion |
| network/profile interaction | obey profile composition ceilings | drop decorative audio first | steal runtime/network reserve |

## Binding Table

| Binding | Canonical source |
|---|---|
| active voice and probe ceilings | `STRATUMX_NUMERIC_SOURCE_OF_TRUTH.md` |
| profile budget envelope | `STRATUMX_PROFILE_COMPOSITION_PROOF.md` |
| material lookup coupling | `levels/l0.5-shared-world-properties/material/43_LOOKUP_MODEL.md` |

## Propagation Tier Matrix

| Tier | Canonical posture | Illegal posture |
|---|---|---|
| near critical cues | preserve first | ambience steals reserve |
| mid-range ambience | degrade before critical cues | fixed-max solve everywhere |
| far-field ambience | degrade earliest | always-on dense solve |

## Local Operating Law

Propagation remains a bounded acoustics solve.
It may not open hidden world-query fanout, hidden probe classes, or profile-blind reserve use.
