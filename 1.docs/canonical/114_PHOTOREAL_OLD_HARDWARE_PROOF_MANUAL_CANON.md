# Photoreal Old Hardware Proof Manual Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Provide the stitched reading spine for photoreal quality, lighting, shadow ladders, fallback law, and old-floor proof so visual ambition and hardware honesty stay coupled.

## Scope of this manual
This manual stitches:
- root `20`, `61`, `65`, `69`, `109`, `114`;
- engine `86–92`, `103–104`, `126–130`, `137–139`;
- sdk `64–67`, `72–73`, `79–84`, `88`;
- tooling `67–68`, `72`, `74`, `84`, `87`, `90–92`;
- editor `101–103`, `115`, `123`, `124`, `126`, `130`, `135`.

## Reading order
1. capability and old-floor law from root `20`, `61`, `69`;
2. backend seam and feature ladder from engine `103–104`;
3. lighting, atmosphere, shading, audio-space continuity, and budgets from engine `86–92`, `126–130`, `137–138`;
4. certification and fallback packets in sdk `79–84`, `88`;
5. tooling benchmark and certification conveyor;
6. editor photoreal and tunnel proof labs.

## Mandatory exact outcomes
- every visual claim has a named fallback ladder;
- tunnel, flashlight, transient light, and shadow proof are explicit;
- old-floor proof is scene-class based rather than vague quality prose;
- first-playable proof region can inherit the same visual law in a narrow scope.

## Current posture
`document_gold / stitched_manual_closed`


## v31 photoreal first-lane linkage
Photoreal proof now starts with the Graphics Port first lane defined in root `122` and engine `145–148`.
The old-hardware proof manual must consume backend feature tiers, disabled optional features, residency rung, shadow tier, and capture artifacts from the Graphics Port packet families in sdk `89–91`.


---
# V32 Old-Floor Visual Proof Closure

## Minimum old-floor fallback rows
| Feature | High path later | Old-floor path now | Required packet |
|---|---|---|---|
| shadows | cascaded/high resolution | single low-res or disabled verdict | `feature.shadow.degraded` |
| local lights | clustered/many lights | capped local light count | `feature.local_light.capped` |
| transient light | full muzzle flash lighting | one-frame simple light or emissive fallback | `feature.transient_light.degraded` |
| terrain textures | high-res mips | lower mip/checker fallback | `resource.texture.degraded` |
| sky | advanced atmosphere | gradient/basic atmosphere | `feature.sky.degraded` |
| post | advanced post stack | exposure+tonemap only | `feature.post.degraded` |
| capture | image+metadata+timing | metadata-only if image unavailable | `capture.degraded` |

## Proof rule
A feature is not photoreal-gold until it has a normal path, old-floor fallback, visible editor badge, SDK packet, tooling evidence, and golden-frame expectation.
