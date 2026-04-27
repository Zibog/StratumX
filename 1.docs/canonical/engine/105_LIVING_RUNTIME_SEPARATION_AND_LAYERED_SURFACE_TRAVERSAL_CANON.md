# Living Runtime Separation And Layered Surface Traversal Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze the runtime boundary between general world material law and living-model consequence law.

This document exists so the archive stops mixing:
- static-world destruction;
- garment and skin layering;
- tissue/bone exposure;
- wound/species consequence.

The goal is one material grammar, but two runtimes with clean handoff.

## Core separation law
Living models must reuse the same material language as the rest of the world.
They may not reuse the same runtime authority as buildings, roads, or props.

The split is mandatory:
- material packages still own matter identity and branch law;
- the living runtime owns host-bound layered traversal, exposure, and handoff to wound/species consequence;
- wound/species canons own lethality, sever legality, bleed/shock, and biological verdicts.

## Exact truth objects
- `LivingHostLayerStack`
- `GarmentLayerBinding`
- `OuterSurfaceExposureState`
- `LayerTraversalLedger`
- `LivingSupportRegionMap`
- `ExposureToWoundHandoff`
- `LivingRecoveryAnchor`

## Host-bound layer law
A living host may bind ordered material packages.
Examples:
- coat -> shirt -> skin -> tissue -> bone;
- armor plate -> padding -> garment -> skin -> tissue -> bone;
- hide -> soft tissue -> bone.

These package layers are lawful material truth.
Their traversal, exposure, and binding to the animated host are lawful living-runtime truth.
They are not independent free-floating rigid entities by default.

## Exact phase order
1. resolve host-bound layer stack and current modifiers;
2. traverse incoming event through ordered layers;
3. publish tear / puncture / burn / perforation / ricochet side effects per layer;
4. update exposure state and region map;
5. hand off biological consequence to engine `67` when required;
6. retain recovery anchor and replay linkage.

## Canonical event classes
The living runtime must normalize at minimum:
- contact and scrape;
- puncture and penetration;
- blunt and blast overpressure;
- burn / char / thermal exposure;
- wetness / contamination style surface events;
- sever / dismember preconditions as handoff signals, not material-owned verdicts.

## Coupling boundaries
| Boundary role | Declared links | Forbidden shortcut |
|---|---|---|
| reads | root `84`, root `96–98`, engine `63`, engine `71`, engine `96` | never owns species AI, morale, or narrative logic |
| publishes | engine `67`, engine `76`, editor `72`, root capture/compare routes | may not bypass wound/species consequence with one-off animation or gore scripts |

## Armor and garment law
Armor, coats, shirts, straps, and similar worn layers are legal material packages bound to the living host.
They may absorb, deflect, tear, perforate, char, soak, or expose lower layers.
They may not become hidden second skeletons or free-floating truth owners.

## Region and support law
Living support regions are legal runtime abstractions for chest, abdomen, limb segments, head regions, joints, and other declared zones.
They are allowed to influence traversal and exposure.
They may not replace material package identity or wound/species law.

## Cheapness law
The living runtime may reduce richness by tier.
It may:
- reduce cosmetic layer detail;
- reduce secondary debris richness;
- compact replay mirrors;
- simplify non-critical visual exposure detail.

It may not:
- skip ordered traversal;
- change stop / perforate / expose / handoff result class silently;
- fuse garment, skin, tissue, and bone into one fake layer when exact traversal is required.

## Certification duties
- retain ordered layer traversal digest;
- retain first exposure/handoff board for failed and recovered runs;
- publish whether biological consequence was or was not invoked;
- prove that cheapness changes never rewrote traversal result class without publication.

## Current posture
`document_gold / closes_living_runtime_separation_gap`
