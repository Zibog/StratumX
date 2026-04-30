# Material Visual Response Family Registry Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze the mandatory registry for material-owned visual response families so damage visuals, overlays, aftermath, debris reveal, and material-derived VFX no longer rely on distributed vocabulary.

## Registry law
- material law owns the semantic class of visual consequence;
- VFX, overlays, shader reveals, decals, residue layers, and spawned media must bind to one canonical family id;
- editor and runtime must read the same family vocabulary;
- cheap visual downgrade may reduce density or lifetime but may not change family identity.

## Mandatory family groups
| Group | Stable id prefix | Meaning |
|---|---|---|
| Reveal / exposure | `matresp.visual.reveal.*` | exposed substrate, fracture edge, interior layer reveal |
| Debris emission | `matresp.visual.debris.*` | dust, clods, splinters, shards, flakes, chunks |
| Overlay / residue | `matresp.visual.overlay.*` | scorch, soot, mud, blood, damp film, frost film |
| Break / tear event | `matresp.visual.break.*` | shatter burst, tear line, buckle ripple, collapse plume |
| Continuous state signal | `matresp.visual.state.*` | dampness darkening, charring progression, moss aging |
| Cheap visual posture | `matresp.visual.cheap.*` | full media, reduced spawn, overlay-only, atlas-only |

## Required row fields
- `visual_family_id`
- `group`
- `source_trigger_classes`
- `source_state_inputs`
- `emit_or_overlay_class`
- `density_semantics`
- `lifetime_semantics`
- `amplitude_semantics`
- `material_archetype_compat`
- `surface_family_modifiers`
- `state_overlay_coupling`
- `light_coupling_refs`
- `audio_coupling_refs` when synced impacts are mandatory
- `cheap_downgrade_law`
- `editor_previewability`
- `validation_field_refs`

## Normalization examples
- `matresp.visual.debris.shards_glass_common`
- `matresp.visual.overlay.scorch_rigid_mineral`
- `matresp.visual.break.tear_fabric_heavy`
- `matresp.visual.reveal.substrate_after_plaster_break`

## Preview contract
Material-centric preview must support:
- bullet hit visual family preview
- blast visual family preview
- burn progression preview
- wetness overlay preview
- aftermath persistence preview
- cheap visual rung preview
