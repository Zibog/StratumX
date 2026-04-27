# UI Runtime Composition Text and HUD Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own runtime UI composition, text layout, HUD publication, and interaction-surface legality.
This file freezes engine-side runtime truth.
Upper layers may request, route, observe, compare, capture, and certify this family.
Upper layers may not replace runtime UI truth with local approximations.

## Truth objects
- `UiCompositionState`
- `TextLayoutState`
- `HudPublicationState`
- `InteractionSurfaceState`

## Runtime phases
1. collect ui model
2. compose layout tree
3. resolve text and interaction regions
4. publish hud/text state
5. emit diagnostics and focus state

## Diagnostics and failure classes
- `TEXT_LAYOUT_FAILURE`
- `HUD_PUBLICATION_DENIED`
- `INTERACTION_SURFACE_LOST`
- `UI_FALLBACK_RUNG_DROP`

## Degradation and fallback law
- degrade is legal only through explicit rung policy;
- each rung drop must publish a typed reason;
- fallback may preserve operator understanding only if it does not fake a higher-quality runtime result;
- a successful observation may not be published when the runtime phase failed and no legal fallback exists.

## Forbidden shortcuts
- editor may not present local debug UI as runtime truth;
- tooling may not suppress engine denial codes;
- sdk may not normalize away layout or interaction-surface distinctions;
- no hidden text or hud publication shortcut may bypass routed legality.

## Exact editor entrypoints
- UI command rows in `editor/110` via `editor/98`

## Required publications
- `obs.ui.*` with layout tree, text state, hud publication verdict, interaction-surface state, and first blocker code;
- retained artifact lineage for baseline/current/recovered bundles that stays reachable from `editor/103`, `editor/105`, and `editor/109`.

## Phase-4 brutal proof slices
- `ui_layout_text_and_hud_publication_remain_truthful`;
- `interaction_surface_loss_publishes_disable_reason`;
- `runtime_ui_debug_overlay_matches_engine_publication`.

## Old-floor evidence obligations
- one retained bundle proving UI degrade keeps the first blocker visible;
- one compare digest proving baseline/current/recovered layout truth;
- one denial sample proving interaction-surface loss is routed rather than locally guessed.

## Current posture
`document_gold / doc_closed_impl_open / runtime_family_closed_in_docs`
