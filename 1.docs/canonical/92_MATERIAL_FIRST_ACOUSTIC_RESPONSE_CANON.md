# Material First Acoustic Response Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze acoustic response as a first-class derivative of material law so sound-family selection, state modifiers, hollowness, leakage, occlusion, and cheap substitution are driven by material truth rather than ad hoc audio-only logic.

## Core law
- `acoustic_profile_ref` is subordinate to material truth and may specialize but not contradict material-owned acoustic family identity;
- material state overlays may remap acoustic modifiers only through canonical remap rules;
- structure, thickness, cavity, and void posture may participate as lawful modifiers when declared by material-bearing structure truth;
- cheap audio substitution must remain material-semantic.

## Mandatory acoustic classes
| Class | Stable id prefix | Meaning |
|---|---|---|
| Impact family | `matresp.audio.impact.*` | what a direct hit sounds like |
| Break family | `matresp.audio.break.*` | crack, shatter, tear, crumble, buckle |
| Surface traversal family | `matresp.audio.traversal.*` | footsteps, scrape, drag, roll |
| Occlusion / leakage family | `matresp.audio.occlusion.*` | through-wall loss, crack leakage, porous leak |
| Resonance / hollowness family | `matresp.audio.resonance.*` | cavity, sheet ring, solid thud, damped mass |
| State remap family | `matresp.audio.state.*` | wet dulling, frozen sharpening, char muffling |
| Cheap acoustic posture | `matresp.audio.cheap.*` | full propagation, classified shortcut, single-shot substitute |

## Required fields
- `acoustic_family_id`
- `impact_family`
- `break_family`
- `traversal_family`
- `occlusion_family`
- `resonance_family`
- `state_remap_refs`
- `thickness_modifier_class`
- `void_or_cavity_modifier_class`
- `damage_state_modifier_class`
- `fallback_family_ref`
- `cheap_runtime_rungs`
- `preview_contract_ref`
- `editor_owner_surface`
- `validation_rule_family`

## Editor obligations
The editor must support:
- assign emitter class from world authoring surface;
- bind reverb/zone profile from world or material-centric surface;
- preview audibility from free camera;
- preview obstruction vs occlusion;
- preview indoor/outdoor transition;
- inspect bus/ducking relation when material-triggered audio reaches mix policy.
