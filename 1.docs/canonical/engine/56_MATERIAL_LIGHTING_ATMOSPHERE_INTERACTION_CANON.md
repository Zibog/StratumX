# Material, Lighting, and Atmosphere Interaction Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes how material descriptors, lighting inputs, atmosphere, weather-facing state, and environment conditions interact at presentation time without violating ownership law.

## Ownership law
Material authoring truth is not lighting truth.
Lighting truth is not atmosphere truth.
These domains meet only through explicitly declared extracted inputs and shared substrate state.

## Canonical interaction classes
| Interaction class | Upstream inputs | Result role |
|---|---|---|
| material_surface_response | material archetype, instance parameters, wetness/heat/burn state | frame-facing surface behavior |
| direct_light_response | lights, shadows, local exposure policy | per-frame lit presentation |
| atmosphere_scattering_response | sky, fog, volumetric, cloud state | global view coloration and attenuation |
| weather_surface_modulation | wetness, precipitation, dust, char, frost | material appearance modulation |
| damage_or_burn_visual_modulation | damage memory, scorch or fracture aftermath | consequence-facing visual state |

## Prohibition
Graphics may not invent hidden wetness, heat, or burn truth merely to look good.
Those states must come from legal world, field, material, or aftermath truth owners.

## Quality-tier law
A quality tier may change fidelity, not semantic ownership.
Low-tier fog, reflections, shadows, or wet-surface treatment remain legal only if the degradation posture is explicit and the resulting presentation still respects canonical state ownership.

## Required diagnostics
The stack must be able to explain, at minimum:
- which material instance or surface class was active;
- which light or atmosphere class dominated the result;
- whether wetness, burn, or dust modulation was active;
- which graphics degradation posture changed the visible outcome.

## Current posture
Sky/environment presentation and material authoring routes exist at the canon level.
Full coupled evidence for wetness/thermal/burn visual propagation is still wider than the currently proved code path.
