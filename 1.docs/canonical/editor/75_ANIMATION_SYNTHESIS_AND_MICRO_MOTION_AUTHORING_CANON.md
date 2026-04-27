# Animation Synthesis And Micro-Motion Authoring Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for animation intent, contact targets, and authored micro-motion legality. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.anim.author_intent_profile` | `route.anim.authoring.author_intent_profile.v1` | `packet.anim.authoring.author_intent_profile.v1` | engine `70` | `mutate.profile` | preview contact intent | `deny.anim.intent_schema_invalid` |
| bind | `btn.anim.bind_contact_target` | `route.anim.authoring.bind_contact_target.v1` | `packet.anim.authoring.bind_contact_target.v1` | engine `70` | `mutate.binding` | bind contact anchor | `deny.anim.contact_target_missing` |
| inspect | `btn.anim.inspect_micro_motion` | `route.anim.authoring.inspect_micro_motion.v1` | `packet.anim.authoring.inspect_micro_motion.v1` | engine `70` | `read.inspect` | micro-motion drilldown | `deny.anim.inspect_scope_invalid` |
| simulate | `btn.anim.simulate_contact_preview` | `route.anim.authoring.simulate_contact_preview.v1` | `packet.anim.authoring.simulate_contact_preview.v1` | engine `70` | `simulate.preview` | preview contact solve | `deny.anim.preview_window_invalid` |
| compare | `btn.anim.compare_contact_triplet` | `route.anim.authoring.compare_contact_triplet.v1` | `packet.anim.authoring.compare_contact_triplet.v1` | engine `70` | `analyze.compare` | contact compare digest | `deny.anim.compare_baseline_missing` |
| capture | `btn.anim.capture_contact_evidence` | `route.anim.authoring.capture_contact_evidence.v1` | `packet.anim.authoring.capture_contact_evidence.v1` | engine `70` | `capture.artifact` | contact evidence bundle | `deny.anim.capture_target_missing` |
| recover | `btn.anim.recover_motion_baseline` | `route.anim.authoring.recover_motion_baseline.v1` | `packet.anim.authoring.recover_motion_baseline.v1` | engine `70` | `recover.baseline` | recovered motion baseline | `deny.anim.recovery_anchor_missing` |
| certify | `btn.anim.certify_motion_pack` | `route.anim.authoring.certify_motion_pack.v1` | `packet.anim.authoring.certify_motion_pack.v1` | engine `70` | `release.certify` | motion pack verdict | `deny.anim.certification_gap` |

## Exact compare modes
- contact triplet compare;
- micro-motion envelope compare;
- authoring profile compare;
- recovery compare;

## Required overlays and drilldowns
- intent spline;
- contact anchor envelope;
- footfall timing;
- micro-motion drift heatmap;
- baseline anchors;

## Exact inspector fields
- intent profile id;
- contact target id;
- solve window id;
- baseline id;
- failed-run id;
- recovery-run id;
- first failure code;

## Disabled reason families
- `disable.anim.target_missing`;
- `disable.anim.profile_unbound`;
- `disable.anim.baseline_missing`;
- `disable.anim.recovery_anchor_missing`;

## Focus and recovery law
- author/bind success -> stay in editor `75` with dirty profile and binding drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `75` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.anim.contact_triplet` and pack `pack.animation_contact_authoring` pinned;
- capture success -> `103` or `105` with capture mode `capture.anim.contact_evidence` and retained artifact refs visible;
- recover success -> rerun compare in editor `75` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.animation_contact_authoring`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.anim.profile`;
- `artifact.anim.binding`;
- `artifact.anim.compare_digest`;
- `artifact.anim.trace_ref`;
- `artifact.anim.baseline_ptr`;

## Freeze relevance
- certification for `pack.animation_contact_authoring` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
