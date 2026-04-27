# Placeholder Prohibition, Stub Gate, and Fake Result Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
runtime rules that forbid fake closure and silent placeholder substitution

## Truth objects
- placeholder flags
- stub verdicts
- fake-result blocks

## Runtime phase order
1. detect placeholder
2. gate publication
3. publish explicit verdict

## Input and output carriers
- inputs arrive from upstream runtime truth, legal bridge surfaces, or internal schedules only;
- outputs publish typed observations, frame/audio/runtime artifacts, or diagnostics;
- ad-hoc side channels are forbidden.

## Diagnostics and observability
- stub detected
- fake closure blocked
- expiry missing

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
