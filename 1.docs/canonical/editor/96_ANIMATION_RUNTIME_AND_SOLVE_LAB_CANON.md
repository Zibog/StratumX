# Animation Runtime And Solve Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for runtime pose solve, contact resolution, and animation legality. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.anim.runtime.author_runtime_profile` | `route.anim.runtime.author_runtime_profile.v1` | `packet.anim.runtime.author_runtime_profile.v1` | engine `96` | `mutate.profile` | runtime profile revision | `deny.animruntime.profile_invalid` |
| bind | `btn.anim.runtime.bind_solve_targets` | `route.anim.runtime.bind_solve_targets.v1` | `packet.anim.runtime.bind_solve_targets.v1` | engine `96` | `mutate.binding` | solve target binding | `deny.animruntime.target_missing` |
| inspect | `btn.anim.runtime.inspect_contact_solve` | `route.anim.runtime.inspect_contact_solve.v1` | `packet.anim.runtime.inspect_contact_solve.v1` | engine `96` | `read.inspect` | contact-solve drilldown | `deny.animruntime.solve_missing` |
| simulate | `btn.anim.runtime.simulate_runtime_preview` | `route.anim.runtime.simulate_runtime_preview.v1` | `packet.anim.runtime.simulate_runtime_preview.v1` | engine `96` | `simulate.preview` | runtime preview run | `deny.animruntime.preview_scope_invalid` |
| compare | `btn.anim.runtime.compare_runtime_triplet` | `route.anim.runtime.compare_runtime_triplet.v1` | `packet.anim.runtime.compare_runtime_triplet.v1` | engine `96` | `analyze.compare` | runtime compare digest | `deny.animruntime.compare_baseline_missing` |
| capture | `btn.anim.runtime.capture_runtime_evidence` | `route.anim.runtime.capture_runtime_evidence.v1` | `packet.anim.runtime.capture_runtime_evidence.v1` | engine `96` | `capture.artifact` | runtime evidence bundle | `deny.animruntime.capture_target_missing` |
| recover | `btn.anim.runtime.recover_runtime_baseline` | `route.anim.runtime.recover_runtime_baseline.v1` | `packet.anim.runtime.recover_runtime_baseline.v1` | engine `96` | `recover.baseline` | recovered runtime baseline | `deny.animruntime.recovery_anchor_missing` |
| certify | `btn.anim.runtime.certify_runtime_pack` | `route.anim.runtime.certify_runtime_pack.v1` | `packet.anim.runtime.certify_runtime_pack.v1` | engine `96` | `release.certify` | runtime pack verdict | `deny.animruntime.certification_gap` |

## Exact compare modes
- runtime triplet compare;
- contact solve compare;
- pose drift compare;
- recovery compare;

## Required overlays and drilldowns
- pose graph;
- contact solve map;
- skin drift heatmap;
- runtime anchors;
- baseline anchors;

## Exact inspector fields
- runtime profile id;
- solve target id;
- pose graph id;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;

## Disabled reason families
- `disable.animruntime.profile_missing`;
- `disable.animruntime.targets_unbound`;
- `disable.animruntime.baseline_missing`;
- `disable.animruntime.route_blocked`;

## Focus and recovery law
- author/bind success -> stay in editor `96` with dirty profile and binding drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `96` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.anim.runtime_triplet` and pack `pack.animation_runtime_contact` pinned;
- capture success -> `103` or `105` with capture mode `capture.anim.runtime_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `96` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.animation_runtime_contact`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.anim.runtime.profile`;
- `artifact.anim.runtime.binding`;
- `artifact.anim.runtime.compare_digest`;
- `artifact.anim.runtime.trace_ref`;
- `artifact.anim.runtime.baseline_ptr`;

## Freeze relevance
- certification for `pack.animation_runtime_contact` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.


## Phase-4 brutal proof slices
- door-handle micro-motion resolves through runtime contact solve instead of canned middle motion;
- recover reruns the same solve-target family and keeps first fallback rung visible.

## Old-floor evidence obligations
- one retained baseline pointer for the last-good rung;
- one compare digest proving current vs baseline vs recovered state;
- one denial or degrade sample with the first blocker code visible to the operator;
- one capture bundle whose artifact lineage remains reachable from `editor/103`, `editor/105`, and `editor/109`;

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
