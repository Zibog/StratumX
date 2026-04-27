# Heavy Domain Capture Replay Compare And Consumer Constitution Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the packet and consumer contract for retained heavy-domain capture, replay, and compare surfaces.

## Mandatory retained bundle rows
| Artifact | Must include |
|---|---|
| capture bundle | scenario id, time window, packet families, producer versions |
| replay bundle | deterministic input digest, scope ids, divergence anchors |
| compare bundle | baseline ref, run ref, first divergence, tolerance posture |
| certification bundle | verdict, blocker family, retained evidence refs |
| release-seal review bundle | sync digest, open-truth rows, readiness verdict |

## Consumer law
Every retained bundle must name:
- primary editor consumer surfaces;
- primary tooling compare/certification routes;
- schema compatibility class;
- forward/backward compatibility posture;
- allowed retention horizon.

## Compare law
A compare tool may not treat a missing packet family as “no change”.
Missing required evidence is a blocker, not a green result.

## Failure families
- `CAPTURE_FAM_*`
- `REPLAY_SCOPE_*`
- `COMPARE_BASELINE_*`
- `CERT_ART_*`
