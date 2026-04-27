# Population Schedule Needs And Faction Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for population schedules, need pressure, crime escalation, faction posture, and base formation. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.soc.author_need_curve` | `route.soc.author_need_curve.v1` | `packet.soc.author_need_curve.v1` | engine `64` | `mutate.profile` | need-curve revision | `deny.soc.need_curve_invalid` |
| bind | `btn.soc.bind_faction_rule` | `route.soc.bind_faction_rule.v1` | `packet.soc.bind_faction_rule.v1` | engine `64` | `mutate.binding` | faction-rule binding | `deny.soc.faction_rule_missing` |
| inspect | `btn.soc.inspect_reason_chain` | `route.soc.inspect_reason_chain.v1` | `packet.soc.inspect_reason_chain.v1` | engine `64` | `read.inspect` | reason-chain drilldown | `deny.soc.reason_chain_missing` |
| simulate | `btn.soc.simulate_population_step` | `route.soc.simulate_population_step.v1` | `packet.soc.simulate_population_step.v1` | engine `64` | `simulate.preview` | schedule/need/crime step preview | `deny.soc.preview_scope_invalid` |
| compare | `btn.soc.compare_triplet` | `route.soc.compare_triplet.v1` | `packet.soc.compare_triplet.v1` | engine `64` | `analyze.compare` | society compare digest | `deny.soc.compare_baseline_missing` |
| capture | `btn.soc.capture_evidence` | `route.soc.capture_evidence.v1` | `packet.soc.capture_evidence.v1` | engine `64` | `capture.artifact` | society evidence bundle | `deny.soc.capture_target_missing` |
| recover | `btn.soc.recover_baseline` | `route.soc.recover_baseline.v1` | `packet.soc.recover_baseline.v1` | engine `64` | `recover.baseline` | recovered society baseline | `deny.soc.recovery_anchor_missing` |
| certify | `btn.soc.certify_pack` | `route.soc.certify_pack.v1` | `packet.soc.certify_pack.v1` | engine `64` | `release.certify` | society pack verdict | `deny.soc.certification_gap` |

## Exact compare modes
- need-pressure triplet compare;
- crime escalation compare;
- faction/base shift compare;
- recovery compare;

## Required overlays and drilldowns
- need curves;
- crime pressure field;
- faction/base legality board;
- reason chain;
- baseline anchors;

## Exact inspector fields
- need curve id;
- faction rule id;
- crime pressure state;
- reason chain id;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;

## Disabled reason families
- `disable.soc.need_curve_missing`;
- `disable.soc.faction_rule_unbound`;
- `disable.soc.baseline_missing`;
- `disable.soc.route_blocked`;

## Phase-3 brutal proof slices
- scarcity raises need pressure until crime posture changes;
- base formation appears only after faction legality allows it;
- recovery restores the previous legality anchor instead of mutating hidden state;
- every outcome exports one reason fragment to `engine/76`.

## Focus and recovery law
- author/bind success -> stay in editor `69` with dirty profile and faction drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `69` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.soc.reason_triplet` and pack `pack.population_tactics_damage` pinned;
- capture success -> `103` or `105` with capture mode `capture.soc.reason_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `69` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.population_tactics_damage`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.soc.need_curve`;
- `artifact.soc.binding`;
- `artifact.soc.compare_digest`;
- `artifact.soc.reason_trace_ref`;
- `artifact.soc.baseline_ptr`;

## Freeze relevance
- certification for `pack.population_tactics_damage` is freeze-relevant if the lab produces retained artifacts that participate in tactics, economy, or quest mixed packs;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
