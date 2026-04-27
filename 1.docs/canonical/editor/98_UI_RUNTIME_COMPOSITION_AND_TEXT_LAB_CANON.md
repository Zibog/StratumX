# UI Runtime Composition And Text Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for runtime UI composition, text, and legal interaction surface authoring. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.ui.author_layout_profile` | `route.ui.author_layout_profile.v1` | `packet.ui.author_layout_profile.v1` | engine `95` | `mutate.profile` | layout profile revision | `deny.ui.profile_invalid` |
| bind | `btn.ui.bind_runtime_surface` | `route.ui.bind_runtime_surface.v1` | `packet.ui.bind_runtime_surface.v1` | engine `95` | `mutate.binding` | runtime surface binding | `deny.ui.surface_missing` |
| inspect | `btn.ui.inspect_composition_tree` | `route.ui.inspect_composition_tree.v1` | `packet.ui.inspect_composition_tree.v1` | engine `95` | `read.inspect` | composition-tree drilldown | `deny.ui.tree_missing` |
| simulate | `btn.ui.simulate_layout_preview` | `route.ui.simulate_layout_preview.v1` | `packet.ui.simulate_layout_preview.v1` | engine `95` | `simulate.preview` | layout preview run | `deny.ui.preview_scope_invalid` |
| compare | `btn.ui.compare_layout_triplet` | `route.ui.compare_layout_triplet.v1` | `packet.ui.compare_layout_triplet.v1` | engine `95` | `analyze.compare` | layout compare digest | `deny.ui.compare_baseline_missing` |
| capture | `btn.ui.capture_ui_evidence` | `route.ui.capture_ui_evidence.v1` | `packet.ui.capture_ui_evidence.v1` | engine `95` | `capture.artifact` | ui evidence bundle | `deny.ui.capture_target_missing` |
| recover | `btn.ui.recover_ui_baseline` | `route.ui.recover_ui_baseline.v1` | `packet.ui.recover_ui_baseline.v1` | engine `95` | `recover.baseline` | recovered ui baseline | `deny.ui.recovery_anchor_missing` |
| certify | `btn.ui.certify_ui_pack` | `route.ui.certify_ui_pack.v1` | `packet.ui.certify_ui_pack.v1` | engine `95` | `release.certify` | ui pack verdict | `deny.ui.certification_gap` |

## Exact compare modes
- layout triplet compare;
- text rendering compare;
- interaction surface compare;
- recovery compare;

## Required overlays and drilldowns
- composition tree;
- text atlas;
- interaction regions;
- hud layers;
- baseline anchors;

## Exact inspector fields
- layout profile id;
- runtime surface id;
- composition tree id;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;

## Disabled reason families
- `disable.ui.profile_missing`;
- `disable.ui.surface_unbound`;
- `disable.ui.baseline_missing`;
- `disable.ui.route_blocked`;

## Focus and recovery law
- author/bind success -> stay in editor `98` with dirty profile and binding drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `98` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.ui.layout_triplet` and pack `pack.ui_runtime_layout_text` pinned;
- capture success -> `103` or `105` with capture mode `capture.ui.layout_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `98` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.ui_runtime_layout_text`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.ui.profile`;
- `artifact.ui.binding`;
- `artifact.ui.compare_digest`;
- `artifact.ui.trace_ref`;
- `artifact.ui.baseline_ptr`;

## Freeze relevance
- certification for `pack.ui_runtime_layout_text` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.


## Phase-4 brutal proof slices
- runtime layout, text, and hud composition stay truthful under play-mode pressure;
- ui evidence may not hide first failure code or interaction-surface loss behind a local editor approximation.

## Old-floor evidence obligations
- one retained baseline pointer for the last-good rung;
- one compare digest proving current vs baseline vs recovered state;
- one denial or degrade sample with the first blocker code visible to the operator;
- one capture bundle whose artifact lineage remains reachable from `editor/103`, `editor/105`, and `editor/109`;

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
