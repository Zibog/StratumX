# Advanced Ballistics Armor Penetration And Wind Drift Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own finite high-velocity penetrator entities, projectile flight, field-influenced trajectory, layered material traversal, ricochet legality, residual-energy publication, and downstream consequence.

## Exact truth objects
- `projectile_state`
- `penetrator_profile`
- `flight_field_sample_state`
- `penetration_window_state`
- `armor_response_state`
- `layer_traversal_state`
- `wind_drift_state`
- `ricochet_state`
- `secondary_impact_state`
- `impact_restore_anchor`

## Exact phase order
1. accept legal projectile or penetrator spawn;
2. integrate flight and field influence;
3. evaluate impact and material-stack entry;
4. traverse lawful layers until stop / embed / perforate / ricochet / deform-only verdict;
5. publish ballistic outcome, residual-energy state, and restore anchor.

## Penetrator-profile law
A lawful penetrator entity must declare:
- mass bucket;
- caliber or profile class;
- shape class;
- drag class;
- stability class;
- penetration class;
- ricochet class;
- optional fragment / split posture;
- field-response flags.

This remains engine technology law, not weapon-design prose.

## Field-influence law
During flight the engine may sample only the field families declared legal for the active tier:
- wind;
- medium density;
- gravity anomaly;
- pressure or turbulence band;
- precipitation / wetness interaction;
- other declared high-velocity field families.

Sampling more fields is legal only if the active cheapness policy allows it.

## Layer-traversal law
Impact resolution must treat material stacks as ordered layers.
For each layer the engine must be able to answer:
- entry angle;
- effective thickness;
- material package and modifiers;
- energy loss;
- deformation or fracture side effects;
- whether a lawful exit trajectory remains.

Static structures, props, and living hosts may all participate in ordered traversal.
Living-host traversal must hand off through engine `105` when the stack is host-bound.

## Canonical result classes
Every advanced-ballistics route must normalize the following result classes:
- `result.stop`
- `result.embed`
- `result.ricochet`
- `result.partial_perforation`
- `result.full_perforation`
- `result.deform_only`
- `result.fracture_without_perforation`

## Ricochet and residual-energy law
Ricochet is legal only when the engine publishes:
- the ricochet verdict;
- the new trajectory state;
- residual energy class;
- downstream secondary impact eligibility.

A secondary impact may then strike a new target lawfully, including another structure, prop, or living target.
This is a consequence chain, not a weapon-only special case.

## Coupling boundaries
| Boundary role | Declared links | Forbidden shortcut |
|---|---|---|
| reads | 62 wind field bands; 48 combat inputs; 67 wound consequence rules; root `96–98` material runtime docs; engine `105` for host-bound living traversal | never owns wound lethality or tactical morale truth |
| publishes | 49 sdk combat packets; 67 wound chain inputs; 76 cause fragments; 50 destruction/aftermath inputs | may not bypass material or armor response tables |

## Resource envelope
- CPU: green <= 1.0 ms, yellow <= 1.6 ms, orange <= 2.2 ms, red > 2.2 ms.
- GPU: none except overlays.
- RAM: green <= 112 MiB projectile history, orange > 176 MiB, red > 256 MiB.
- disk / IO: artifact and checkpoint only.

## Ordered degrade ladder
- reduce non-critical ballistic debug detail;
- compact historical projectile mirrors;
- reduce retained compare mirrors before any solver change;
- sample overlay only outside active combat bubble;
- reduce secondary cosmetic effects before changing stop / perforate / ricochet truth.

## Replay and compare windows
- `baseline.120`;
- `cert.300`;
- `restore.300`;
- `salvo.600`;

## Failure and denial code families
- `ballistics.penetration.rule_missing`
- `ballistics.wind.drift_diverge`
- `ballistics.ricochet.illegal`
- `ballistics.layer.traversal_missing`
- `ballistics.restore.anchor_missing`

## Certification duties
- retain flight/impact compare and penetration digest per certification pack;
- publish first failure code when armor, layer traversal, or ricochet rule fails;
- retain restore anchor for persisted projectile-affecting outcomes;
- prove that degraded posture never silently changes stop / perforate / ricochet verdict classes.

## Current posture
`document_gold / doc_closed_impl_open`
