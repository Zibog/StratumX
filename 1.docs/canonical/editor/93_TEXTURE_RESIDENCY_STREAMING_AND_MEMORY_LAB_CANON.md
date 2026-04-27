# Texture Residency Streaming And Memory Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for texture residency, memory ceilings, and streaming authoring. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.render.residency.author_residency_profile` | `route.render.residency.author_residency_profile.v1` | `packet.render.residency.author_residency_profile.v1` | engine `89` | `mutate.profile` | residency profile revision | `deny.residency.profile_invalid` |
| bind | `btn.render.residency.bind_stream_group` | `route.render.residency.bind_stream_group.v1` | `packet.render.residency.bind_stream_group.v1` | engine `89` | `mutate.binding` | stream group binding | `deny.residency.group_missing` |
| inspect | `btn.render.residency.inspect_residency_pressure` | `route.render.residency.inspect_residency_pressure.v1` | `packet.render.residency.inspect_residency_pressure.v1` | engine `89` | `read.inspect` | pressure drilldown | `deny.residency.pressure_missing` |
| simulate | `btn.render.residency.simulate_residency_preview` | `route.render.residency.simulate_residency_preview.v1` | `packet.render.residency.simulate_residency_preview.v1` | engine `89` | `simulate.preview` | residency preview run | `deny.residency.preview_scope_invalid` |
| compare | `btn.render.residency.compare_residency_triplet` | `route.render.residency.compare_residency_triplet.v1` | `packet.render.residency.compare_residency_triplet.v1` | engine `89` | `analyze.compare` | residency compare digest | `deny.residency.compare_baseline_missing` |
| capture | `btn.render.residency.capture_residency_evidence` | `route.render.residency.capture_residency_evidence.v1` | `packet.render.residency.capture_residency_evidence.v1` | engine `89` | `capture.artifact` | residency evidence bundle | `deny.residency.capture_target_missing` |
| recover | `btn.render.residency.recover_residency_baseline` | `route.render.residency.recover_residency_baseline.v1` | `packet.render.residency.recover_residency_baseline.v1` | engine `89` | `recover.baseline` | recovered residency baseline | `deny.residency.recovery_anchor_missing` |
| certify | `btn.render.residency.certify_residency_pack` | `route.render.residency.certify_residency_pack.v1` | `packet.render.residency.certify_residency_pack.v1` | engine `89` | `release.certify` | residency pack verdict | `deny.residency.certification_gap` |

## Exact compare modes
- residency triplet compare;
- old-floor ladder compare;
- pressure rung compare;
- recovery compare;

## Required overlays and drilldowns
- stream groups;
- mip residency;
- pressure ladder;
- old-floor rung;
- baseline anchors;

## Exact inspector fields
- residency profile id;
- stream group id;
- pressure class;
- baseline id;
- failed-run id;
- recovery-run id;
- old-floor rung;

## Disabled reason families
- `disable.residency.group_missing`;
- `disable.residency.baseline_missing`;
- `disable.residency.capture_forbidden`;
- `disable.residency.route_blocked`;

## Focus and recovery law
- author/bind success -> stay in editor `93` with dirty profile and binding drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `93` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.render.residency_triplet` and pack `pack.texture_residency_memory` pinned;
- capture success -> `103` or `105` with capture mode `capture.render.residency_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `93` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.texture_residency_memory`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.render.residency.profile`;
- `artifact.render.residency.binding`;
- `artifact.render.residency.compare_digest`;
- `artifact.render.residency.trace_ref`;
- `artifact.render.residency.baseline_ptr`;

## Freeze relevance
- certification for `pack.texture_residency_memory` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.


## Phase-4 brutal proof slices
- 4K residency pressure drops by lawful rung with typed reason publication;
- mip/sampler fallback never impersonates a higher residency tier than the one actually bound.

## Old-floor evidence obligations
- one retained baseline pointer for the last-good rung;
- one compare digest proving current vs baseline vs recovered state;
- one denial or degrade sample with the first blocker code visible to the operator;
- one capture bundle whose artifact lineage remains reachable from `editor/103`, `editor/105`, and `editor/109`;

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
