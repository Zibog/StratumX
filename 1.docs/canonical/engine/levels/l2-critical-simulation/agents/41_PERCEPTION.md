# Perception

## Role

Bounded perception solve.

## Perception Matrix

| Concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| near/far posture | obey simulation tiers | degrade far perception first | far perception consumes near reserve |
| query count | bounded by profile/family ceiling | collapse low-value sensing first | unbounded visibility-style fanout |
| export | shaped bounded records only | drop optional detail | raw sensor dump |

## Sensing Matrix

| Sensing class | Canonical posture | Illegal posture |
|---|---|---|
| near critical sensing | preserve first | far sensing steals reserve |
| far/background sensing | degrade first | full-fidelity sensing everywhere |
| export shape | bounded records only | raw sensor dump |

## Cache and Miss Matrix

| Surface | Canonical posture | Failure posture | Illegal posture |
|---|---|---|---|
| perception cache | bounded local cache only | evict far detail first | unbounded historical cache |
| miss fallback | explicit coarse fallback classes | approximate before widening query fanout | retry-until-hit scan loops |
| cross-agent sharing | shaped squad/local sharing only | drop optional shared hints | hidden global perception bus |

## Input / Output Law

| Surface | Legal input | Legal output | Illegal output |
|---|---|---|---|
| world read set | local tier-legal descriptors only | bounded perception facts | raw traversal dump |
| acoustics / visibility hints | shaped bridge tokens only | bounded fused fact set | hidden ownership of foreign-family state |
| far-agent posture | coarse sensing classes | schedule/decision-ready hints | near-fidelity full-state mirror |

## Rule

Perception respects simulation tiers and bounded sensing classes. Far perception degrades before near entitlement is consumed.

## Local Operating Law

Perception is a bounded sensing contract.
It may not become a visibility clone, a full-world scan, a raw telemetry exporter, or a hidden shared-state bus.
