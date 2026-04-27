# Upload Staging

## Role

Imaging-side contract for staging and upload consumption.

## Staging Matrix

| Concern | Canonical law | Failure posture | Illegal posture |
|---|---|---|---|
| upload class | bounded runtime-owned upload classes only | drop optional uploads first | imaging creates new upload class |
| staging lifetime | bounded by fence/release law | discard failed optional upload | persistent staging slab ownership |
| per-path distinction | textures, geometry, frame resources remain separated | degrade largest optional class first | shared hidden upload pool |
| readback | diagnostics-only | cancel diagnostics first | readback in critical path |

## Rule

Upload staging is consumed by imaging but owned by runtime-resource services.

## Failure Posture

Texture uploads, geometry uploads, and transient frame-resource uploads remain separate classes. When pressure rises, optional class uploads drop before the low-latency path is widened.

## Illegal Patterns

- merged hidden upload pool;
- runtime decode widening through staging;
- readback inserted into critical upload path.

## Per-Path Matrix

| Upload path | Canonical class separation | Illegal merge |
|---|---|---|
| texture uploads | distinct bounded class | hidden shared slab |
| geometry uploads | distinct bounded class | hidden shared slab |
| transient frame-resource uploads | distinct bounded class | hidden shared slab |
| diagnostics readback | diagnostics-only | critical upload coupling |

## Failure Priority Matrix

Texture, geometry, and transient frame-resource uploads degrade independently.
A breach in one class may not silently widen another class or steal queue reserve from the frame path.

## Local Operating Law

Upload staging is a local consumer contract, not a local ownership surface.
All staging ceilings and release windows remain `L1.5` law.
