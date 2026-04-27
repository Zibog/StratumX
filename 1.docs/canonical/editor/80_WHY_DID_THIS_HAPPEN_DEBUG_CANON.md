# Why Did This Happen Production Causality Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for causality explanation, blocker provenance, and legal recovery mapping. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.reason.author_explanation_scope` | `route.reason.author_explanation_scope.v1` | `packet.reason.author_explanation_scope.v1` | engine `76` | `mutate.profile` | explanation scope revision | `deny.reason.scope_invalid` |
| bind | `btn.reason.bind_trace_policy` | `route.reason.bind_trace_policy.v1` | `packet.reason.bind_trace_policy.v1` | engine `76` | `mutate.binding` | trace binding graph | `deny.reason.trace_policy_missing` |
| inspect | `btn.reason.inspect_causal_chain` | `route.reason.inspect_causal_chain.v1` | `packet.reason.inspect_causal_chain.v1` | engine `76` | `read.inspect` | causal-chain drilldown | `deny.reason.chain_missing` |
| simulate | `btn.reason.simulate_causal_replay` | `route.reason.simulate_causal_replay.v1` | `packet.reason.simulate_causal_replay.v1` | engine `76` | `simulate.preview` | causal replay preview | `deny.reason.preview_scope_invalid` |
| compare | `btn.reason.compare_reason_triplet` | `route.reason.compare_reason_triplet.v1` | `packet.reason.compare_reason_triplet.v1` | engine `76` | `analyze.compare` | reason compare digest | `deny.reason.compare_baseline_missing` |
| capture | `btn.reason.capture_reason_evidence` | `route.reason.capture_reason_evidence.v1` | `packet.reason.capture_reason_evidence.v1` | engine `76` | `capture.artifact` | causal evidence bundle | `deny.reason.capture_target_missing` |
| recover | `btn.reason.recover_reason_baseline` | `route.reason.recover_reason_baseline.v1` | `packet.reason.recover_reason_baseline.v1` | engine `76` | `recover.baseline` | recovered reason baseline | `deny.reason.recovery_anchor_missing` |
| certify | `btn.reason.certify_reason_pack` | `route.reason.certify_reason_pack.v1` | `packet.reason.certify_reason_pack.v1` | engine `76` | `release.certify` | causality pack verdict | `deny.reason.certification_gap` |

## Exact compare modes
- reason chain compare;
- baseline vs failed compare;
- recovery mapping compare;
- causal replay compare;

## Required overlays and drilldowns
- reason chain;
- blocking codes;
- recovery map;
- source route ids;
- baseline anchors;

## Exact inspector fields
- trace policy id;
- route id;
- blocker code;
- baseline id;
- failed-run id;
- recovery-run id;
- next legal recovery action;

## Disabled reason families
- `disable.reason.chain_missing`;
- `disable.reason.policy_unbound`;
- `disable.reason.baseline_missing`;
- `disable.reason.route_blocked`;

## Focus and recovery law
- author/bind success -> stay in editor `80` with dirty profile and binding drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `80` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.reason.triplet` and pack `pack.causality_reason_chain` pinned;
- capture success -> `103` or `105` with capture mode `capture.reason.bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `80` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.causality_reason_chain`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.reason.profile`;
- `artifact.reason.binding`;
- `artifact.reason.compare_digest`;
- `artifact.reason.trace_ref`;
- `artifact.reason.baseline_ptr`;

## Freeze relevance
- certification for `pack.causality_reason_chain` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.



## Phase-3 operator questions that must resolve here
- why did society shift into crime or new base formation?
- why did tactics replan after cover break?
- why did migration reroute?
- why is the path blocked?
- why did inventory scarcity change the trader state?
- why did the quest branch or semantic denial occur?

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
