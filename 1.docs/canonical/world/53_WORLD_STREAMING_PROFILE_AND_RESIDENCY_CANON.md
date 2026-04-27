# World Streaming Profile And Residency Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Freeze how world packages declare streaming and residency posture for chunks, placements, materials, and environment assets.

## Streaming profile law
A world package may bind a streaming profile, but that binding must remain explicit and validation-visible.
The profile must declare at minimum:
- chunk residency policy;
- warm/hot radius posture;
- old-floor degrade posture;
- rebuild and save interaction with streamed-out chunks.

## Residency law
Residency is not purely graphics-owned.
World must declare which world data may be cold, streamed, warmed, pinned, or evicted without breaking legality or restore safety.

## Deny conditions
- missing streaming profile for a world that requires one;
- resident/cold state that makes validation or save ambiguous;
- old-floor posture that is undeclared or inconsistent with build/runtime proof.
