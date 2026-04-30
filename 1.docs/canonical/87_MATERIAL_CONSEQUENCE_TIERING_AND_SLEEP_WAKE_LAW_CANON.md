# Material Consequence Tiering And Sleep Wake Law Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze the cheap-but-honest runtime law that lets a world material sleep with full behavioral meaning and wake only when a legal trigger demands consequence.

This document closes the gap between "nearly free world simulation" and "fake reaction chains".

## Core law
A material system is cheap when idle because it stores law, not active solve.
A material system is honest when awake because it executes the correct response profile for the active trigger and publishes any persistent downstream truth.

## Canonical tiers
| Tier | Name | Meaning |
|---|---|---|
| `tier.0` | dormant | no active solve; stack and overlays are present but sleeping |
| `tier.1` | contact_wake | short-lived local reaction to hit/contact/tool use |
| `tier.2` | local_consequence | local crater/crack/tear/dent/shatter solve and immediate overlay update |
| `tier.3` | downstream_consequence | publish aftermath into nav/cover/audio/visibility/wetness/fire/world state |
| `tier.4` | far_echo` | retain only summary state needed for distant or restored correctness |

## Sleep-wake triggers
| Trigger class | Minimum wake tier |
|---|---|
| `trigger.ballistic_hit` | `tier.1` |
| `trigger.blast_overpressure` | `tier.2` |
| `trigger.fire_exposure` | `tier.2` |
| `trigger.wetness_contact` | `tier.1` or `tier.2` depending on profile |
| `trigger.tool_dig_or_cut` | `tier.2` |
| `trigger.structural_overload` | `tier.2` then `tier.3` if aftermath matters |
| `trigger.fall_or_collision` | `tier.1` or `tier.2` |
| `trigger.restore_or_stream_in` | `tier.4` summary restore, then promote if interaction resumes |

## Tier law
- dormant state may not silently mutate consequence truth;
- a wake trigger may promote only to the minimum legal tier first;
- the runtime may skip fidelity, not semantics;
- tier reduction must follow explicit degrade law and may not erase persistent aftermath.

## Cheap response examples
### Bullet through heavy cloth
- wake to `tier.1`
- small deformation + bullet hole + cloth motion impulse
- publish only persistent hole/tear overlay if profile says so
- return toward `tier.0` when no continuing forces remain

### Grenade on asphalt
- wake to `tier.2`
- local crack/depression/shard solve
- if road usability or cover changes, promote aftermath publication to `tier.3`
- retain only resulting state at `tier.4`

### Rain on absorbent soil
- wake to `tier.1` for wetness accumulation
- promote to `tier.2` only if mud/churn thresholds or traction law change
- publish new traversal or splash state to `tier.3` when world consequence becomes durable

## Required downstream truth publication
When local consequence changes any of the following, the runtime must publish explicit downstream truth:
- traversal or traction legality;
- cover or line-of-sight legality;
- acoustic leakage or occlusion class;
- fire, wetness, or thermal propagation eligibility;
- persistent material identity or surface-family transition.

## Forbidden shortcuts
- global full-material solve as the default idle posture;
- hidden renderer-only aftermath without world publication;
- keeping a material forever awake because a cheap tiered law was never defined;
- using one "damage happened" bit instead of tier-specific consequence classes.

## Required diagnostics
The stack must be able to answer:
- which tier the object is currently in;
- which trigger woke it;
- which response profile fired;
- whether downstream truth was published;
- which degrade rung limited fidelity.

## Current posture
`document_gold / closes_cheap_material_consequence_gap / no_demo_content`

## Cheap-runtime unification
Sleep/wake and consequence tiers are not standalone optimization trivia.
They are a branch of the material-owned cheap-runtime contract finalized in root `93` and must govern physics, visual, audio, light, and persistence posture together.
