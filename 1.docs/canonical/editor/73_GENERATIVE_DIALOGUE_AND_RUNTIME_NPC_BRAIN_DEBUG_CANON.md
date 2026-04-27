# Generative Dialogue And Runtime NPC Brain Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for grounding policy, guarded dialogue, world-consequence binding, denial-path inspection, outcome simulation, recovery, and certification. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.semantic.author_grounding_policy` | `route.semantic.author_grounding_policy.v1` | `packet.semantic.author_grounding_policy.v1` | engine `68` | `mutate.profile` | grounding-policy revision | `deny.semantic.grounding_policy_invalid` |
| bind | `btn.semantic.bind_world_consequence` | `route.semantic.bind_world_consequence.v1` | `packet.semantic.bind_world_consequence.v1` | engine `68` | `mutate.binding` | world-consequence binding | `deny.semantic.world_consequence_missing` |
| inspect | `btn.semantic.inspect_denial_path` | `route.semantic.inspect_denial_path.v1` | `packet.semantic.inspect_denial_path.v1` | engine `68` | `read.inspect` | denial-path drilldown | `deny.semantic.denial_path_missing` |
| simulate | `btn.semantic.simulate_outcome` | `route.semantic.simulate_outcome.v1` | `packet.semantic.simulate_outcome.v1` | engine `68` | `simulate.preview` | guarded semantic outcome preview | `deny.semantic.preview_scope_invalid` |
| compare | `btn.semantic.compare_outcome` | `route.semantic.compare_outcome.v1` | `packet.semantic.compare_outcome.v1` | engine `68` | `analyze.compare` | semantic outcome digest | `deny.semantic.compare_baseline_missing` |
| capture | `btn.semantic.capture_evidence` | `route.semantic.capture_evidence.v1` | `packet.semantic.capture_evidence.v1` | engine `68` | `capture.artifact` | semantic evidence bundle | `deny.semantic.capture_target_missing` |
| recover | `btn.semantic.recover_baseline` | `route.semantic.recover_baseline.v1` | `packet.semantic.recover_baseline.v1` | engine `68` | `recover.baseline` | recovered semantic baseline | `deny.semantic.recovery_anchor_missing` |
| certify | `btn.semantic.certify_pack` | `route.semantic.certify_pack.v1` | `packet.semantic.certify_pack.v1` | engine `68` | `release.certify` | semantic pack verdict | `deny.semantic.certification_gap` |

## Exact compare modes
- guarded outcome compare;
- denial-path compare;
- memory delta compare;
- recovery compare;

## Required overlays and drilldowns
- grounding policy;
- denial path;
- guard chain;
- outcome triplet;
- baseline anchors;

## Exact inspector fields
- grounding policy id;
- world consequence binding id;
- guard chain id;
- outcome triplet ref;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;

## Disabled reason families
- `disable.semantic.grounding_policy_missing`;
- `disable.semantic.consequence_unbound`;
- `disable.semantic.baseline_missing`;
- `disable.semantic.route_blocked`;

## Phase-3 brutal proof slices
- dialogue either emits lawful world consequence or explicit guarded denial;
- semantic drift cannot certify without baseline/failure/recovery triplet;
- every outcome exports one reason fragment to `engine/76`.

## Focus and recovery law
- author/bind success -> stay in editor `73` with dirty profile and denial-path drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `73` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.semantic.outcome_triplet` and pack `pack.semantic_consequence_chain` pinned;
- capture success -> `103` or `105` with capture mode `capture.semantic.guard_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `73` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.semantic_consequence_chain`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.semantic.policy`;
- `artifact.semantic.binding`;
- `artifact.semantic.compare_digest`;
- `artifact.semantic.trace_ref`;
- `artifact.semantic.baseline_ptr`;

## Freeze relevance
- certification for `pack.semantic_consequence_chain` is freeze-relevant if the lab produces retained artifacts that participate in quest or population mixed packs;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
