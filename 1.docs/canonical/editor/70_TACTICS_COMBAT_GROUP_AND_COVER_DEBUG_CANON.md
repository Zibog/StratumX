# Tactics Combat Group And Cover Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for squad doctrine, cover legality, suppression, flank windows, and destruction-aware replanning. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.tac.author_doctrine` | `route.tac.author_doctrine.v1` | `packet.tac.author_doctrine.v1` | engine `65` | `mutate.profile` | doctrine revision | `deny.tac.doctrine_invalid` |
| bind | `btn.tac.bind_cover_rule` | `route.tac.bind_cover_rule.v1` | `packet.tac.bind_cover_rule.v1` | engine `65` | `mutate.binding` | cover-rule binding | `deny.tac.cover_rule_missing` |
| inspect | `btn.tac.inspect_cover_graph` | `route.tac.inspect_cover_graph.v1` | `packet.tac.inspect_cover_graph.v1` | engine `65` | `read.inspect` | cover-graph drilldown | `deny.tac.cover_graph_missing` |
| simulate | `btn.tac.simulate_cover_break` | `route.tac.simulate_cover_break.v1` | `packet.tac.simulate_cover_break.v1` | engine `65` | `simulate.preview` | suppression/flank/cover-break preview | `deny.tac.preview_scope_invalid` |
| compare | `btn.tac.compare_triplet` | `route.tac.compare_triplet.v1` | `packet.tac.compare_triplet.v1` | engine `65` | `analyze.compare` | tactics compare digest | `deny.tac.compare_baseline_missing` |
| capture | `btn.tac.capture_evidence` | `route.tac.capture_evidence.v1` | `packet.tac.capture_evidence.v1` | engine `65` | `capture.artifact` | tactics evidence bundle | `deny.tac.capture_target_missing` |
| recover | `btn.tac.recover_baseline` | `route.tac.recover_baseline.v1` | `packet.tac.recover_baseline.v1` | engine `65` | `recover.baseline` | recovered tactics baseline | `deny.tac.recovery_anchor_missing` |
| certify | `btn.tac.certify_pack` | `route.tac.certify_pack.v1` | `packet.tac.certify_pack.v1` | engine `65` | `release.certify` | tactics pack verdict | `deny.tac.certification_gap` |

## Exact compare modes
- doctrine triplet compare;
- suppression spike compare;
- cover-break rollback compare;
- flank legality compare;

## Required overlays and drilldowns
- doctrine board;
- suppression field;
- cover graph;
- flank windows;
- rollback anchors;

## Exact inspector fields
- doctrine id;
- cover rule id;
- cover graph state;
- suppression field state;
- flank window id;
- baseline id;
- failed-run id;
- first blocker code;

## Disabled reason families
- `disable.tac.doctrine_missing`;
- `disable.tac.cover_rule_unbound`;
- `disable.tac.baseline_missing`;
- `disable.tac.route_blocked`;

## Phase-3 brutal proof slices
- destruction invalidates cover and forces lawful replanning;
- suppression and flank windows update without mutating wound truth directly;
- every replanned squad state exports one reason fragment to `engine/76`.

## Focus and recovery law
- author/bind success -> stay in editor `70` with dirty profile and cover drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `70` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.tac.cover_triplet` and pack `pack.population_tactics_damage` pinned;
- capture success -> `103` or `105` with capture mode `capture.tac.cover_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `70` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.population_tactics_damage`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.tac.doctrine`;
- `artifact.tac.binding`;
- `artifact.tac.compare_digest`;
- `artifact.tac.cover_trace_ref`;
- `artifact.tac.baseline_ptr`;

## Freeze relevance
- certification for `pack.population_tactics_damage` is freeze-relevant if the lab produces retained artifacts that participate in wound, destruction, or old-floor mixed packs;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
