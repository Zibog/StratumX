# Visibility, Culling, Occlusion, and Scene Query Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
view-dependent selection of visible candidates and occlusion legality

## Truth objects
- view frustum
- occluders
- candidate set
- visibility verdicts

## Runtime phase order
1. collect candidates
2. cull
3. occlusion test
4. publish visible set

## Input and output carriers
- inputs arrive from upstream runtime truth, legal bridge surfaces, or internal schedules only;
- outputs publish typed observations, frame/audio/runtime artifacts, or diagnostics;
- ad-hoc side channels are forbidden.

## Diagnostics and observability
- visible count
- occlusion misses
- fallback reason

## Degradation law
- degrade is legal only through explicit tiered policy;
- degrade must generate a typed reason;
- degrade may not silently impersonate a higher tier.

## Forbidden shortcuts
- no scene-specific hardcoding in technology truth;
- no editor-local override of runtime truth without routed legality;
- no placeholder path without explicit stub verdict.

## Evidence obligations
- proof of typed publication;
- proof of diagnostics emission;
- proof of degrade reason publication;
- proof of boundary preservation.

## Current posture
This document freezes engine-side technology law. Broad implementation proof remains open unless stated otherwise in the implementation ledger.
