# Advanced Wounding Gore And Species Rule Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for anatomy, armor stacking, ballistic trace, ordered living-layer traversal, species-specific wound law, replay compare, recovery, and certification. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.wound.author_anatomy_profile` | `route.wound.author_anatomy_profile.v1` | `packet.wound.author_anatomy_profile.v1` | engine `67` + engine `71` + engine `105` | `mutate.profile` | anatomy-profile revision | `deny.wound.anatomy_profile_invalid` |
| bind | `btn.wound.bind_armor_stack` | `route.wound.bind_armor_stack.v1` | `packet.wound.bind_armor_stack.v1` | engine `67` + engine `71` + engine `105` | `mutate.binding` | armor-stack binding | `deny.wound.armor_stack_missing` |
| inspect | `btn.wound.inspect_trace` | `route.wound.inspect_trace.v1` | `packet.wound.inspect_trace.v1` | engine `67` + engine `71` + engine `105` | `read.inspect` | wound-trace drilldown | `deny.wound.trace_missing` |
| simulate | `btn.wound.simulate_test_salvo` | `route.wound.simulate_test_salvo.v1` | `packet.wound.simulate_test_salvo.v1` | engine `67` + engine `71` + engine `105` | `simulate.preview` | ballistic/wound preview | `deny.wound.preview_scope_invalid` |
| compare | `btn.wound.compare_replay` | `route.wound.compare_replay.v1` | `packet.wound.compare_replay.v1` | engine `67` + engine `71` + engine `105` | `analyze.compare` | wound replay digest | `deny.wound.compare_baseline_missing` |
| capture | `btn.wound.capture_evidence` | `route.wound.capture_evidence.v1` | `packet.wound.capture_evidence.v1` | engine `67` + engine `71` + engine `105` | `capture.artifact` | wound evidence bundle | `deny.wound.capture_target_missing` |
| recover | `btn.wound.recover_baseline` | `route.wound.recover_baseline.v1` | `packet.wound.recover_baseline.v1` | engine `67` + engine `71` + engine `105` | `recover.baseline` | recovered wound baseline | `deny.wound.recovery_anchor_missing` |
| certify | `btn.wound.certify_pack` | `route.wound.certify_pack.v1` | `packet.wound.certify_pack.v1` | engine `67` + engine `71` + engine `105` | `release.certify` | wound pack verdict | `deny.wound.certification_gap` |

## Exact compare modes
- ballistic replay compare;
- armor penetration compare;
- living-layer traversal compare;
- species wound-law compare;
- recovery compare;

## Required overlays and drilldowns
- anatomy profile;
- armor stack;
- ordered living-layer stack;
- penetration trace;
- replay triplet;
- baseline anchors;

## Exact inspector fields
- anatomy profile id;
- armor stack id;
- living layer stack id;
- trace id;
- replay triplet ref;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;

## Disabled reason families
- `disable.wound.anatomy_profile_missing`;
- `disable.wound.armor_stack_unbound`;
- `disable.wound.baseline_missing`;
- `disable.wound.route_blocked`;

## Phase-3 brutal proof slices
- armor stacking changes wound outcome without cheating outside equipment truth;
- ordered layer traversal through garment/skin/tissue/bone stays explainable through engine `105`;
- species-specific wound law stays explainable through `engine/76`;
- replay compare keeps baseline/failure/recovery aligned.

## Focus and recovery law
- author/bind success -> stay in editor `72` with dirty profile and trace drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `72` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.wound.replay_triplet` and pack `pack.ballistics_wound_trace` pinned;
- capture success -> `103` or `105` with capture mode `capture.wound.trace_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `72` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.ballistics_wound_trace`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.wound.profile`;
- `artifact.wound.binding`;
- `artifact.wound.compare_digest`;
- `artifact.wound.trace_ref`;
- `artifact.wound.baseline_ptr`;

## Freeze relevance
- certification for `pack.ballistics_wound_trace` is freeze-relevant if the lab produces retained artifacts that participate in tactics or old-floor mixed packs;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
