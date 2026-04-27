# Procedural Motion Generation Contact Solve And Style Variation Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own intermediate motion generation between authored anchors and final pose.

## Exact truth objects

| Object | Role |
|---|---|
| `MotionPriorSet` | family of allowed generated motion priors per body class |
| `ContactTargetIntent` | where a hand, foot, body, or tool is trying to go |
| `IntermediateMotionSolve` | generated in-between trajectory and balance solve |
| `StyleVariationDescriptor` | allowed stylistic variance envelope |
| `BodyClassModifier` | human, mutant, animal, armored, injured, or burdened bias |
| `FinalPoseAuthorityVerdict` | whether final pose remained inside declared envelopes |

## Solve families
- door handle and lever reach
- cover lean and blind fire
- climb, vault, breach, and stumble
- recoil, pain reaction, and injury-compensated locomotion

## Authority split
Authored anchors and contact intents are truth.
Intermediate motion is generated truth constrained by style, body class, injury state, and legality envelope.
Final skinning may not erase a failed contact solve.

## Publications
- `packet.living.motion_solve.v1`
- `packet.living.contact_result.v1`
- `packet.living.pose_authority.v1`

## Failure families
- `motion.intermediate_solve_missing`
- `motion.contact_target_illegal`
- `motion.style_variation_out_of_envelope`
- `motion.final_pose_authority_break`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
