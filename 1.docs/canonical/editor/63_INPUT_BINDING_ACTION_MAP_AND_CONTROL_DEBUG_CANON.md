# Input Binding, Action Map, and Control Debug Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document defines the editor-facing workflow for input binding, action-map inspection, control-context switching, and control diagnostics.

## Workflow
`open bindings/control surface -> inspect or edit action map -> validate conflicts and context law -> preview/test control path -> inspect focus or denial diagnostics -> commit or revert`

## Required surfaces
| Surface | Role |
|---|---|
| binding editor | map device inputs to named actions and analog controls |
| context inspector | view active contexts such as editor, UI, simulate, play, cinematic |
| control debug panel | active maps, denied inputs, focus owner, latency posture, preview traces |
| apply/revert surface | explicit transition from preview binding state to durable profile |

## Conflict law
A binding change is not complete unless the editor can name any conflict, ambiguity, or illegal context overlap.

## Current posture
This document closes the editor answer layer for control-configuration asks.
