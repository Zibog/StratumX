# World Field Substrate And Update Order Constitution Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define one field substrate for wetness, heat, smoke, toxicity, wind, obscuration, sound hints, and anomaly carriers.

## Field carriers
| Field | Canonical carrier | Storage law |
|---|---|---|
| wetness | scalar surface/cell field | surface first, cell summary second |
| heat | scalar volume/surface field | cell + local hot patch |
| smoke density | scalar volume field | volume with settling summary |
| toxicity | scalar field | cell/volume with source tags |
| wind | vector field | region driver + local perturbation |
| obscuration | scalar field | derived from smoke/fog/dust families |
| sound hint | scalar hint field | debug/assist only, never final audio truth |
| anomaly field | typed field | explicit family id required |

## Update order
environment input -> source injection -> diffusion/advection -> local overrides -> threshold transitions -> consequence publication.

## Conflict law
Derived fields may not overwrite owner fields.
Local overrides expire by explicit rule, not by hidden cache replacement.
