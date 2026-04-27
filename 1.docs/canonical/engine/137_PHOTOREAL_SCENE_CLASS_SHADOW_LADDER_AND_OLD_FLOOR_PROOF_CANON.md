# Photoreal Scene Class Shadow Ladder And Old Floor Proof Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Make shadows, transient emission, fog/media, and visibility fallbacks scene-class exact.

## Scene classes

| Scene class | Required proof behavior |
|---|---|
| `tunnel_firefight` | flashlight, muzzle flash, smoke, debris, and audio continuity remain readable |
| `wet_forest_dusk` | foliage shadowing, wetness readability, and depth layering remain readable |
| `storm_horizon` | distant cloud wall, lightning, and horizon observability remain readable |
| `interior_camp` | mixed local light, warmth, and occlusion transitions remain readable |
| `destruction_block` | dust, smoke, transient light, and collapse aftermath remain readable |

## Shadow ladder
| Tier | Meaning |
|---|---|
| `shadow.full_near` | near exact dynamic participation |
| `shadow.reduced_mid` | reduced update or reduced receiver set |
| `shadow.signature_far` | signature-only participation |
| `shadow.denied` | legal only with explicit diagnostic verdict |

## Tunnel proof packet set
The tunnel scene must publish:
- transient light count verdict,
- active shadow tier,
- fog/media tier,
- muzzle-flash participation verdict,
- first blocked feature,
- capture artifact ref.

## Failure law
A photoreal proof run fails if the scene is visually green but packetized fallback evidence is missing.
