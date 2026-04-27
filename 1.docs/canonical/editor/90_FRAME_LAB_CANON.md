# Frame Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for frame budgets, frame timing, and capture legality. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.render.frame.author_frame_budget` | `route.render.frame.author_frame_budget.v1` | `packet.render.frame.author_frame_budget.v1` | engine `86` | `mutate.profile` | frame budget revision | `deny.frame.budget_invalid` |
| bind | `btn.render.frame.bind_frame_scope` | `route.render.frame.bind_frame_scope.v1` | `packet.render.frame.bind_frame_scope.v1` | engine `86` | `mutate.binding` | frame scope binding | `deny.frame.scope_missing` |
| inspect | `btn.render.frame.inspect_frame_timing` | `route.render.frame.inspect_frame_timing.v1` | `packet.render.frame.inspect_frame_timing.v1` | engine `86` | `read.inspect` | frame timing drilldown | `deny.frame.timing_missing` |
| simulate | `btn.render.frame.simulate_frame_preview` | `route.render.frame.simulate_frame_preview.v1` | `packet.render.frame.simulate_frame_preview.v1` | engine `86` | `simulate.preview` | frame preview run | `deny.frame.preview_scope_invalid` |
| compare | `btn.render.frame.compare_frame_triplet` | `route.render.frame.compare_frame_triplet.v1` | `packet.render.frame.compare_frame_triplet.v1` | engine `86` | `analyze.compare` | frame compare digest | `deny.frame.compare_baseline_missing` |
| capture | `btn.render.frame.capture_frame_evidence` | `route.render.frame.capture_frame_evidence.v1` | `packet.render.frame.capture_frame_evidence.v1` | engine `86` | `capture.artifact` | frame evidence bundle | `deny.frame.capture_target_missing` |
| recover | `btn.render.frame.recover_frame_baseline` | `route.render.frame.recover_frame_baseline.v1` | `packet.render.frame.recover_frame_baseline.v1` | engine `86` | `recover.baseline` | recovered frame baseline | `deny.frame.recovery_anchor_missing` |
| certify | `btn.render.frame.certify_frame_pack` | `route.render.frame.certify_frame_pack.v1` | `packet.render.frame.certify_frame_pack.v1` | engine `86` | `release.certify` | frame pack verdict | `deny.frame.certification_gap` |

## Exact compare modes
- frame triplet compare;
- timing rung compare;
- budget ladder compare;
- old-floor compare;

## Required overlays and drilldowns
- frame timeline;
- GPU/CPU bars;
- budget ladders;
- baseline anchors;
- capture provenance;

## Exact inspector fields
- frame budget id;
- scope id;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;
- old-floor rung;

## Disabled reason families
- `disable.frame.scope_missing`;
- `disable.frame.baseline_missing`;
- `disable.frame.capture_forbidden`;
- `disable.frame.route_blocked`;

## Focus and recovery law
- author/bind success -> stay in editor `90` with dirty profile and binding drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `90` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.render.frame_triplet` and pack `pack.frame_budget_triplet` pinned;
- capture success -> `103` or `105` with capture mode `capture.render.frame_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `90` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.frame_budget_triplet`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.render.frame.profile`;
- `artifact.render.frame.binding`;
- `artifact.render.frame.compare_digest`;
- `artifact.render.frame.trace_ref`;
- `artifact.render.frame.baseline_ptr`;

## Freeze relevance
- certification for `pack.frame_budget_triplet` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.


## Phase-4 brutal proof slices
- frame budget and timing remain inspectable under mixed load;
- old-floor rung drop preserves truthful timing publication and denial evidence.

## Old-floor evidence obligations
- one retained baseline pointer for the last-good rung;
- one compare digest proving current vs baseline vs recovered state;
- one denial or degrade sample with the first blocker code visible to the operator;
- one capture bundle whose artifact lineage remains reachable from `editor/103`, `editor/105`, and `editor/109`;

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
