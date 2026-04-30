# Render Resource Residency Upload And Streaming Pressure Canon

**Stack version:** `SX-CANON/1.0.26/STACK-v32`

## Purpose
Define resource upload, residency, streaming pressure, and fallback law for the graphics port and first photoreal lane.

## Resource classes
- vertex buffer;
- index buffer;
- uniform/constant buffer;
- storage buffer;
- 2D texture;
- texture array/atlas;
- render target;
- depth target;
- sampler;
- shader module;
- pipeline object;
- capture readback target.

## Upload law
Every upload publishes:
- source asset id;
- target resource handle;
- byte count;
- staging path;
- residency ticket;
- backend class;
- failure code if blocked.

## Residency law
Texture/material residency must support:
- missing texture fallback;
- low mip fallback;
- memory pressure warnings;
- old-floor texture rung;
- capture of actual resident mips.

## Streaming pressure
Renderer must publish:
- requested bytes;
- resident bytes;
- evicted bytes;
- upload budget used;
- first missing resource;
- visible fallback count.

## Photoreal relevance
4K assets are allowed only when residency law can explain:
- when they are loaded;
- what mips are resident;
- what fallback is visible;
- whether old-floor profile degrades legally.

## Current posture
`document_gold / resource_residency_defined / implementation_open`


---
# V32 Resource Closure

## Required resource classes
- proof/static mesh buffers;
- terrain tile buffers;
- material constants;
- camera/frame uniforms;
- base color textures;
- normal maps;
- roughness/metal/AO textures;
- sky resources;
- shadow targets;
- output color/depth targets;
- debug overlay buffers;
- capture/readback resources.

## Residency states
`not_requested`, `requested`, `staging`, `resident`, `evicted`, `missing`, `failed`, `fallback_resident`.

## Missing-resource fallbacks
Texture -> checker texture. Normal -> flat normal. Material -> missing-material shader. Mesh -> proof mesh only in debug. Sky -> gradient. Shadow target -> disabled shadow verdict.
