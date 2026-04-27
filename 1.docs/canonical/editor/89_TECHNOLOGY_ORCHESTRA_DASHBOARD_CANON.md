# Technology Orchestra Dashboard Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This dashboard is not advisory-only.
It is the top production control surface for:
- pressure visibility;
- pack status;
- route state;
- artifact completeness;
- freeze blockers;
- old-floor rung;
- proof-region relay completeness;
- launch readiness and first-result status;
- regression posture;
- region-scale validation posture.

## Law
Every dashboard action must map to a real button id in `editor/110`.
It may not invent a second signoff, launch, certification, or regression path outside the retained canonical packs.

## Mandatory widgets
- retained bundle lineage;
- canonical pack status board;
- blocker matrix;
- old-floor rung board;
- regression board;
- region-validation board;
- first-result verification board.

## Negative law
The dashboard may summarize many domains.
It may not hide the first blocker, merge canonical packs into unnamed buckets, or replace retained evidence with a synthetic summary.
