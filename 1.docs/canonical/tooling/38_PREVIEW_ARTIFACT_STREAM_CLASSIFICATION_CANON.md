# Preview, Artifact, and Stream Classification Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the result classes that tooling may publish upward.
Its job is to stop preview data, durable artifacts, streams, and diagnostics from blending into one muddy payload concept.

## Result classes
| Class | Meaning | Examples |
|---|---|---|
| preview | discardable speculative or live-inspection result | vertical-slice scene bootstrap view, runtime impact preview |
| projection | immutable structured view of current truth owned elsewhere | entity list, material stack details, world summary |
| stream | bounded event-like feed | runtime events, build progress, diagnostic feed |
| artifact | durable generated/imported product | imported mesh artifact, cooked package, build output |
| diagnostic | explanation of legality, degrade, error, or status | validation result, runtime warning, deferred notice |

## Classification law
- preview is not durable truth;
- artifact is not runtime attachment;
- stream is bounded and cannot silently become background shadow truth;
- diagnostics must remain searchable and attributable;
- a single API call may publish multiple result classes, but each must remain explicitly labeled.

## Current posture note
The uploaded vertical-slice path currently mixes preview/projection/diagnostic classes inside one observation payload.
That is acceptable for a narrow demo, but broader product workflows should preserve the result-class distinction explicitly.
