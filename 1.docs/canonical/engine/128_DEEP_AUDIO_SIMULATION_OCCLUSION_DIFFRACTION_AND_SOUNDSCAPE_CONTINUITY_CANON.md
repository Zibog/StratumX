# Deep Audio Simulation Occlusion Diffraction And Soundscape Continuity Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own exact audio-runtime law for diffraction, layered occlusion, portals, indoors/outdoors, material-first variation, and large-world continuity.

## Exact truth objects

| Object | Role |
|---|---|
| `AudioPropagationPathSet` | active propagation paths from emitter to listener |
| `DiffractionVerdict` | whether edge or aperture diffraction is legal |
| `LayeredOcclusionDigest` | cumulative occlusion result by path segment |
| `PortalTransitionState` | interior/exterior continuity through portals and openings |
| `SurfaceEventVariationSeed` | seeded variation for steps, impacts, and scrapes |
| `SoundscapeContinuityLedger` | far-to-near continuity for weather, fire, crowd, fauna |
| `AudioStreamResidencyLedger` | stream residency and pressure posture |

## Evaluation order
`event classify -> material-pair resolution -> propagation path build -> diffraction / occlusion solve -> portal continuity -> mix / priority -> publication`

## Mandatory publications
- `packet.audio.event_result.v1`
- `packet.audio.occlusion_report.v1`
- `packet.audio.soundscape_continuity.v1`
- `packet.audio.stream_pressure.v1`

## Failure families
- `audio.diffraction_gap`
- `audio.layered_occlusion_break`
- `audio.portal_transition_desync`
- `audio.soundscape_continuity_gap`

## Current posture
`document_gold / runtime_contract_closed / implementation_open`
