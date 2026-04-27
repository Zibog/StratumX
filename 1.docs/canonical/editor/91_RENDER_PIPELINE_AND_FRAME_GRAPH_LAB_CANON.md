# Render Pipeline And Frame Graph Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for render pipeline authoring, pass ownership, and frame-graph legality. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.render.pipeline.author_pipeline_profile` | `route.render.pipeline.author_pipeline_profile.v1` | `packet.render.pipeline.author_pipeline_profile.v1` | engine `86` | `mutate.profile` | pipeline profile revision | `deny.pipeline.profile_invalid` |
| bind | `btn.render.pipeline.bind_frame_graph` | `route.render.pipeline.bind_frame_graph.v1` | `packet.render.pipeline.bind_frame_graph.v1` | engine `86` | `mutate.binding` | frame graph binding | `deny.pipeline.graph_missing` |
| inspect | `btn.render.pipeline.inspect_pass_ownership` | `route.render.pipeline.inspect_pass_ownership.v1` | `packet.render.pipeline.inspect_pass_ownership.v1` | engine `86` | `read.inspect` | pass ownership drilldown | `deny.pipeline.pass_unowned` |
| simulate | `btn.render.pipeline.simulate_frame_graph` | `route.render.pipeline.simulate_frame_graph.v1` | `packet.render.pipeline.simulate_frame_graph.v1` | engine `86` | `simulate.preview` | frame graph preview | `deny.pipeline.preview_scope_invalid` |
| compare | `btn.render.pipeline.compare_frame_graph_triplet` | `route.render.pipeline.compare_frame_graph_triplet.v1` | `packet.render.pipeline.compare_frame_graph_triplet.v1` | engine `86` | `analyze.compare` | frame graph compare digest | `deny.pipeline.compare_baseline_missing` |
| capture | `btn.render.pipeline.capture_frame_graph_evidence` | `route.render.pipeline.capture_frame_graph_evidence.v1` | `packet.render.pipeline.capture_frame_graph_evidence.v1` | engine `86` | `capture.artifact` | frame graph evidence bundle | `deny.pipeline.capture_target_missing` |
| recover | `btn.render.pipeline.recover_frame_graph_baseline` | `route.render.pipeline.recover_frame_graph_baseline.v1` | `packet.render.pipeline.recover_frame_graph_baseline.v1` | engine `86` | `recover.baseline` | recovered frame graph baseline | `deny.pipeline.recovery_anchor_missing` |
| certify | `btn.render.pipeline.certify_pipeline_pack` | `route.render.pipeline.certify_pipeline_pack.v1` | `packet.render.pipeline.certify_pipeline_pack.v1` | engine `86` | `release.certify` | pipeline pack verdict | `deny.pipeline.certification_gap` |

## Exact compare modes
- pass graph compare;
- ownership compare;
- recovery compare;
- old-floor compare;

## Required overlays and drilldowns
- frame graph;
- pass ownership;
- async queue map;
- resource hazards;
- baseline anchors;

## Exact inspector fields
- pipeline profile id;
- frame graph id;
- pass id;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;

## Disabled reason families
- `disable.pipeline.graph_missing`;
- `disable.pipeline.profile_missing`;
- `disable.pipeline.baseline_missing`;
- `disable.pipeline.route_blocked`;

## Focus and recovery law
- author/bind success -> stay in editor `91` with dirty profile and binding drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `91` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.render.frame_graph` and pack `pack.render_pipeline_frame_graph` pinned;
- capture success -> `103` or `105` with capture mode `capture.render.pipeline_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `91` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.render_pipeline_frame_graph`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.render.pipeline.profile`;
- `artifact.render.pipeline.binding`;
- `artifact.render.pipeline.compare_digest`;
- `artifact.render.pipeline.trace_ref`;
- `artifact.render.pipeline.baseline_ptr`;

## Freeze relevance
- certification for `pack.render_pipeline_frame_graph` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.


## Phase-4 brutal proof slices
- pass ownership, attachment legality, and graph ordering remain explicit under stress;
- compare after recover shows the same graph family instead of silently rebuilding a different one.

## Old-floor evidence obligations
- one retained baseline pointer for the last-good rung;
- one compare digest proving current vs baseline vs recovered state;
- one denial or degrade sample with the first blocker code visible to the operator;
- one capture bundle whose artifact lineage remains reachable from `editor/103`, `editor/105`, and `editor/109`;

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
