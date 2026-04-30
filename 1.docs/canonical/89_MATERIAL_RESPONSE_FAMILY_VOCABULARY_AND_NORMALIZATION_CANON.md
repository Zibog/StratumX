# Material Response Family Vocabulary And Normalization Canon

**Stack version:** `SX-CANON/1.0.28/STACK-v34`

## Purpose
Freeze the mandatory normalized vocabulary for material-owned reaction families so the canon no longer stops at `*_profile_ref` ownership and instead names the exact family classes that physics, audio, visual, light, runtime, editor, world, sdk, and tooling must share.

This document exists to end the ambiguity where a material can point at many response references but the archive still lacks one canonical dictionary for what those references must classify.

## Master law
- material law owns the normalized response vocabulary when a world-facing thing is declared material-bearing;
- domain-specific systems may elaborate execution detail, but they may not rename the response family identity;
- terrain, prop, structure, foliage, garment, body, and debris surfaces must resolve through the same family language even when their execution branches differ;
- required family groups are mandatory for active contour authoring; optional family groups may extend but not replace mandatory meaning;
- cheap runtime posture must degrade from this vocabulary, not invent a second low-cost naming system.

## Mandatory family groups
| Family group | Stable id prefix | Owner layer | Required for active contour | Purpose |
|---|---|---|---|---|
| Physical contact response | `matresp.physical.contact.*` | material law -> physics | yes | collision, tool, touch, scrape, contact transfer |
| Penetration response | `matresp.physical.penetration.*` | material law -> ballistics / tools | yes | bullet, shard, spike, puncture legality |
| Fracture and collapse response | `matresp.physical.fracture.*` | material law -> destruction | yes | crack, split, shatter, tear, buckle, collapse |
| Thermal and burn response | `matresp.physical.thermal.*` | material law -> fire / temperature | yes | heat soak, ignition, char, melt, scorch |
| Wetness and hydrology response | `matresp.physical.hydrology.*` | material law -> water / leak | yes | absorb, repel, film-wet, saturate, leak-driven changes |
| Traversal and navigation response | `matresp.runtime.traversal.*` | material law -> nav consequence | yes | footfall legality, slipperiness, blockage, softness |
| Visual response | `matresp.visual.*` | material law -> overlay / VFX | yes | reveal, debris, dust, scorch, residue, break visuals |
| Acoustic response | `matresp.audio.*` | material law -> audio | yes | impact timbre, resonance, hollowness, leakage, occlusion effect |
| Light interaction response | `matresp.light.*` | material law -> graphics | yes | transmission, diffusion, reflectance modulation, shadow breakup |
| Cheap runtime posture | `matresp.runtime.cheap.*` | material law -> runtime orchestration | yes | degrade rung, substitute policy, wake/sleep publication |
| Persistence and aftermath response | `matresp.persistence.*` | material law -> world/persistence | yes | what must survive save/restore and what may stay derived |
| Evidence and preview response | `matresp.proof.*` | material law -> editor/certification | yes | previewability, compare/capture requirements, evidence pack relation |

## Required fields for every family row
Every material response family row must publish at least:
- `family_id`
- `family_group`
- `owner_layer`
- `required_trigger_classes`
- `required_state_inputs`
- `required_output_branches`
- `optional_modifiers`
- `cheap_runtime_rung`
- `compatibility_tags`
- `fallback_family_ref`
- `invalid_combination_refs`
- `editor_preview_mode`
- `persistence_posture`
- `capture_bundle_family`
- `validation_gate_family`

## Mandatory normalization law
### Trigger normalization
All material-owned branches must normalize at least these trigger classes:
- `trigger.contact`
- `trigger.scrape`
- `trigger.ballistic_hit`
- `trigger.blast_overpressure`
- `trigger.structural_stress`
- `trigger.thermal_contact`
- `trigger.fire_exposure`
- `trigger.wetness_contact`
- `trigger.submersion`
- `trigger.cut_or_dig`
- `trigger.light_exposure` when the material branch modulates transmission or state-driven reflectance

### State normalization
All material-owned branches must normalize at least these state modifiers:
- `state.clean`
- `state.wet`
- `state.muddy`
- `state.dusty`
- `state.frozen`
- `state.charred`
- `state.cracked`
- `state.fractured`
- `state.bloodied`
- `state.mossed`
- `state.age_worn`

## Compatibility and fallback law
- a family row must declare which archetype classes it is legal for;
- a surface family may specialize the row but may not erase mandatory outputs;
- a cheap runtime fallback must preserve semantic category even when fidelity drops;
- if no valid family exists, validation must fail rather than silently substitute a visually similar but semantically different branch;
- fallback may move from exact solve to canonical approximation, but never from one material meaning to another.

## Required active family buckets
The active gold contour must ship canonical rows for at least:
- rigid mineral
- brittle glass/ceramic
- fibrous wood
- sheet/thin metal
- heavy structural metal
- granular soil/gravel
- absorbent cloth/fabric
- living foliage/wood
- flesh/bone baseline

## Registry-grade depth requirement for mandatory groups
The active contour is not gold if any mandatory family group exists only as a label without branch-grade law.

### Runtime cheapness groups must also publish
- canonical rung ids
- branch participation mask
- local-solve gate
- immediate vs postponed branch sets
- validator floor relation
- editor publication signal

### Persistence groups must also publish
- retained truth classes
- save/restore scope
- derivable-on-load classes
- aftermath retention policy
- world package storage law
- invalid persistence combinations

### Proof groups must also publish
- preview trigger legality
- compare mode family
- capture bundle family
- retained artifact class
- freeze blocker family
- evidence row class

## Editor and world obligations
- editor must expose the normalized family vocabulary through one material-centric authoring surface;
- world packages may store only canonical family refs and lawful modifiers, not custom branch-owned aliases;
- operator-facing preview labels may remain friendly, but they must map one-way to normalized trigger classes rather than inventing a second vocabulary.

## Required cross-document bindings
- physical master contour: `88_PHYSICAL_RESPONSE_CONSTITUTION_AND_CROSS_DOMAIN_CONSEQUENCE_CANON.md`
- light branch: `90_MATERIAL_LIGHT_TRANSMISSION_AND_SHADOW_RESPONSE_CANON.md`
- visual branch: `91_MATERIAL_VISUAL_RESPONSE_FAMILY_REGISTRY_CANON.md`
- acoustic branch: `92_MATERIAL_FIRST_ACOUSTIC_RESPONSE_CANON.md`
- cheap runtime and route closure: `93_MATERIAL_RUNTIME_CHEAPNESS_AND_CROSS_LAYER_ROUTE_CLOSURE_CANON.md`
- persistence and aftermath registry: `94_MATERIAL_PERSISTENCE_AND_AFTERMATH_RESPONSE_REGISTRY_CANON.md`
- proof/freeze registry: `95_MATERIAL_PROOF_PREVIEW_CAPTURE_AND_FREEZE_REGISTRY_CANON.md`
- editor grammar: `editor/113_MATERIAL_CENTRIC_OPERATOR_SURFACE_CANON.md`
