# Navmesh Path Cover And Traversal Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for navigation legality, cover use, and traversal authoring. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.nav.author_traversal_profile` | `route.nav.author_traversal_profile.v1` | `packet.nav.author_traversal_profile.v1` | engine `72` | `mutate.profile` | traversal profile revision | `deny.nav.profile_invalid` |
| bind | `btn.nav.bind_cover_policy` | `route.nav.bind_cover_policy.v1` | `packet.nav.bind_cover_policy.v1` | engine `72` | `mutate.binding` | cover binding graph | `deny.nav.cover_policy_missing` |
| inspect | `btn.nav.inspect_route_legality` | `route.nav.inspect_route_legality.v1` | `packet.nav.inspect_route_legality.v1` | engine `72` | `read.inspect` | route legality drilldown | `deny.nav.route_missing` |
| simulate | `btn.nav.simulate_path_preview` | `route.nav.simulate_path_preview.v1` | `packet.nav.simulate_path_preview.v1` | engine `72` | `simulate.preview` | preview path run | `deny.nav.preview_scope_invalid` |
| compare | `btn.nav.compare_route_triplet` | `route.nav.compare_route_triplet.v1` | `packet.nav.compare_route_triplet.v1` | engine `72` | `analyze.compare` | route compare digest | `deny.nav.compare_baseline_missing` |
| capture | `btn.nav.capture_route_evidence` | `route.nav.capture_route_evidence.v1` | `packet.nav.capture_route_evidence.v1` | engine `72` | `capture.artifact` | route evidence bundle | `deny.nav.capture_target_missing` |
| recover | `btn.nav.recover_nav_baseline` | `route.nav.recover_nav_baseline.v1` | `packet.nav.recover_nav_baseline.v1` | engine `72` | `recover.baseline` | recovered nav baseline | `deny.nav.recovery_anchor_missing` |
| certify | `btn.nav.certify_nav_pack` | `route.nav.certify_nav_pack.v1` | `packet.nav.certify_nav_pack.v1` | engine `72` | `release.certify` | navigation pack verdict | `deny.nav.certification_gap` |

## Exact compare modes
- route triplet compare;
- cover traversal compare;
- dynamic blocker compare;
- old-floor compare;

## Required overlays and drilldowns
- route graph;
- cover mesh;
- dynamic blockers;
- active bubble;
- baseline anchors;

## Exact inspector fields
- route graph id;
- cover policy id;
- traversal profile id;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;

## Disabled reason families
- `disable.nav.graph_missing`;
- `disable.nav.cover_unbound`;
- `disable.nav.baseline_missing`;
- `disable.nav.route_blocked`;

## Focus and recovery law
- author/bind success -> stay in editor `76` with dirty profile and binding drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `76` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.nav.route_triplet` and pack `pack.navigation_traversal_cover` pinned;
- capture success -> `103` or `105` with capture mode `capture.nav.route_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `76` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.navigation_traversal_cover`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.nav.profile`;
- `artifact.nav.binding`;
- `artifact.nav.compare_digest`;
- `artifact.nav.trace_ref`;
- `artifact.nav.baseline_ptr`;

## Freeze relevance
- certification for `pack.navigation_traversal_cover` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.



## Phase-3 brutal proof slices
- destruction invalidates a previously legal route and forces path rebuild;
- doors, ladders, climb, and cover reservation stay explainable under blocked reasons;
- every rebuild emits one route compare digest and one reason backlink to `editor/80`.

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
