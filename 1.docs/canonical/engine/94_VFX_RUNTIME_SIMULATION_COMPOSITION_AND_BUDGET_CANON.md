# VFX Runtime Simulation, Composition, and Budget Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
event-driven effect truth, composition, and budget governance

## Truth objects
- effect events
- emitters
- sim buffers
- budget state

## Runtime phase order
1. spawn effects
2. simulate
3. compose
4. publish effect output

## Input and output carriers
- inputs arrive from upstream runtime truth, legal bridge surfaces, or internal schedules only;
- outputs publish typed observations, frame/audio/runtime artifacts, or diagnostics;
- ad-hoc side channels are forbidden.

## Diagnostics and observability
- effect counts
- drop reasons
- budget tier

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

## Exact editor entrypoints
- VFX command rows in `editor/110` via `editor/97`

## Required publications
- `obs.vfx.*` with emitter graph, media chain, budget rung, and composite verdict;

## Phase-4 brutal proof slices
- `vfx_and_ui_publish_debug_truth_without_masking_runtime_state`;
- `budget_drop_emits_typed_vfx_rung`;

## Old-floor evidence obligations
- one retained bundle proving the actual composite rung under pressure;
- one denial sample proving dropped effect counts remain visible;

## Current posture
`document_gold / doc_closed_impl_open / runtime_family_closed_in_docs`
