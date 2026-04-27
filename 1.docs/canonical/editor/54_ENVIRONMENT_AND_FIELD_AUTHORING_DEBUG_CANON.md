# Environment and Field Authoring Debug Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
This document freezes the product-layer workflow for sky, atmosphere, wind, wetness, rain, and fire-adjacent field debugging.
The target is not a pretty backdrop.
The target is a debuggable authoring surface for runtime-owned field consequences.

## Required suites and surfaces
The editor must be able to present, over time:
- environment parameters and profiles;
- field overlays or inspectors;
- diagnostics for denied/degraded field states;
- preview/runtime distinction for speculative versus authoritative field effects.

## Core ask classes
- set sky or environment profile;
- adjust wind;
- adjust fog/rain/wetness;
- ignite or suppress a test subject when legal;
- inspect why a field consequence is or is not happening.

## Current implementation posture
Sky/environment viewport delivery has explicit canon support through the first-product documents.
Broader field-authoring and debug closure, especially for coupled fire/weather simulation, remains `specified_not_proved` in the uploaded code anchors examined for this patch.
