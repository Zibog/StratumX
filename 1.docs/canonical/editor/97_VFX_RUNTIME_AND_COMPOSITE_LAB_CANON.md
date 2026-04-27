# VFX Runtime And Composite Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for vfx chains, media composition, and runtime composite authoring. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.vfx.author_composite_profile` | `route.vfx.author_composite_profile.v1` | `packet.vfx.author_composite_profile.v1` | engine `94` | `mutate.profile` | composite profile revision | `deny.vfx.profile_invalid` |
| bind | `btn.vfx.bind_emitter_group` | `route.vfx.bind_emitter_group.v1` | `packet.vfx.bind_emitter_group.v1` | engine `94` | `mutate.binding` | emitter group binding | `deny.vfx.group_missing` |
| inspect | `btn.vfx.inspect_media_chain` | `route.vfx.inspect_media_chain.v1` | `packet.vfx.inspect_media_chain.v1` | engine `94` | `read.inspect` | media-chain drilldown | `deny.vfx.media_missing` |
| simulate | `btn.vfx.simulate_composite_preview` | `route.vfx.simulate_composite_preview.v1` | `packet.vfx.simulate_composite_preview.v1` | engine `94` | `simulate.preview` | composite preview run | `deny.vfx.preview_scope_invalid` |
| compare | `btn.vfx.compare_vfx_triplet` | `route.vfx.compare_vfx_triplet.v1` | `packet.vfx.compare_vfx_triplet.v1` | engine `94` | `analyze.compare` | vfx compare digest | `deny.vfx.compare_baseline_missing` |
| capture | `btn.vfx.capture_vfx_evidence` | `route.vfx.capture_vfx_evidence.v1` | `packet.vfx.capture_vfx_evidence.v1` | engine `94` | `capture.artifact` | vfx evidence bundle | `deny.vfx.capture_target_missing` |
| recover | `btn.vfx.recover_vfx_baseline` | `route.vfx.recover_vfx_baseline.v1` | `packet.vfx.recover_vfx_baseline.v1` | engine `94` | `recover.baseline` | recovered vfx baseline | `deny.vfx.recovery_anchor_missing` |
| certify | `btn.vfx.certify_vfx_pack` | `route.vfx.certify_vfx_pack.v1` | `packet.vfx.certify_vfx_pack.v1` | engine `94` | `release.certify` | vfx pack verdict | `deny.vfx.certification_gap` |

## Exact compare modes
- vfx triplet compare;
- media chain compare;
- budget rung compare;
- recovery compare;

## Required overlays and drilldowns
- emitter graph;
- media chain;
- budget ladder;
- composite stages;
- baseline anchors;

## Exact inspector fields
- composite profile id;
- emitter group id;
- media chain id;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;

## Disabled reason families
- `disable.vfx.profile_missing`;
- `disable.vfx.group_unbound`;
- `disable.vfx.baseline_missing`;
- `disable.vfx.route_blocked`;

## Focus and recovery law
- author/bind success -> stay in editor `97` with dirty profile and binding drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `97` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.vfx.triplet` and pack `pack.vfx_composite_media` pinned;
- capture success -> `103` or `105` with capture mode `capture.vfx.bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `97` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.vfx_composite_media`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.vfx.profile`;
- `artifact.vfx.binding`;
- `artifact.vfx.compare_digest`;
- `artifact.vfx.trace_ref`;
- `artifact.vfx.baseline_ptr`;

## Freeze relevance
- certification for `pack.vfx_composite_media` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.


## Phase-4 brutal proof slices
- vfx media chain stays inspectable from spawn through compose;
- degrade under pressure must emit a typed rung and retained trace ref.

## Old-floor evidence obligations
- one retained baseline pointer for the last-good rung;
- one compare digest proving current vs baseline vs recovered state;
- one denial or degrade sample with the first blocker code visible to the operator;
- one capture bundle whose artifact lineage remains reachable from `editor/103`, `editor/105`, and `editor/109`;

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
