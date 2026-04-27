# Render, Lookdev, and Graphics Routing Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document maps graphics- and lookdev-facing asks into tooling intent classes and lower-stack consequences.

## Canonical route
`editor graphics ask -> tooling lookdev or environment intent -> preview/artifact/diagnostic classification -> sdk bridge only where lower truth or frame policy must change -> engine extraction/presentation consequences -> viewport and diagnostic projections`

## Graphics task classes
| Task class | Tooling result class |
|---|---|
| assign or edit material inputs | authoring transaction |
| adjust lighting, sky, fog, atmosphere | authoring transaction plus preview result |
| request render debug mode | diagnostics-only result |
| validate graphics budget or visibility posture | diagnostics-only result |
| build or package graphics-related artifacts | artifact result |

## Distinction law
Lookdev preview is not the same as durable material or environment truth.
Tooling must clearly distinguish live preview deltas from committed authored state.

## Current posture
This closes the routing language for graphics tasks and aligns it with the new graphics capability matrix.
