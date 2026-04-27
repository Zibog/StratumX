# World-Scale and Streaming Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document names the `L5` surface families needed for large-world residency, region promotion, far-field observation, and streaming diagnostics.

## Surface families
| Surface family | Intended role | Current posture |
|---|---|---|
| `WorldScaleIngressPacket` | request region open, residency promotion, debug focus, and streaming probes | specified_only |
| `WorldScaleObservation` | publish residency tiers, pressure, region summaries, and far-field visibility facts | specified_only |
| `WorldScaleControl` | refresh, pin, or snapshot large-world focus under legality gates | specified_only |
| `WorldScaleHandle` | bind long-lived world/region context without exposing lower truth | specified_only |

## Law
Large-world debug and authoring must cross the bridge through explicit packet, observation, control, and handle classes rather than hidden caches.
