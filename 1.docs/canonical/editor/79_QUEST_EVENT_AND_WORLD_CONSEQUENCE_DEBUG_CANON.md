# Quest Event And World Consequence Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for quest chains, systemic events, and persistent world consequence authoring. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.quest.author_quest_profile` | `route.quest.author_quest_profile.v1` | `packet.quest.author_quest_profile.v1` | engine `75` | `mutate.profile` | quest profile revision | `deny.quest.profile_invalid` |
| bind | `btn.quest.bind_consequence_policy` | `route.quest.bind_consequence_policy.v1` | `packet.quest.bind_consequence_policy.v1` | engine `75` | `mutate.binding` | consequence binding graph | `deny.quest.policy_missing` |
| inspect | `btn.quest.inspect_checkpoint_chain` | `route.quest.inspect_checkpoint_chain.v1` | `packet.quest.inspect_checkpoint_chain.v1` | engine `75` | `read.inspect` | checkpoint chain drilldown | `deny.quest.chain_missing` |
| simulate | `btn.quest.simulate_event_branch` | `route.quest.simulate_event_branch.v1` | `packet.quest.simulate_event_branch.v1` | engine `75` | `simulate.preview` | event-branch preview | `deny.quest.preview_scope_invalid` |
| compare | `btn.quest.compare_consequence_triplet` | `route.quest.compare_consequence_triplet.v1` | `packet.quest.compare_consequence_triplet.v1` | engine `75` | `analyze.compare` | consequence compare digest | `deny.quest.compare_baseline_missing` |
| capture | `btn.quest.capture_consequence_evidence` | `route.quest.capture_consequence_evidence.v1` | `packet.quest.capture_consequence_evidence.v1` | engine `75` | `capture.artifact` | consequence evidence bundle | `deny.quest.capture_target_missing` |
| recover | `btn.quest.recover_consequence_baseline` | `route.quest.recover_consequence_baseline.v1` | `packet.quest.recover_consequence_baseline.v1` | engine `75` | `recover.baseline` | recovered consequence baseline | `deny.quest.recovery_anchor_missing` |
| certify | `btn.quest.certify_quest_pack` | `route.quest.certify_quest_pack.v1` | `packet.quest.certify_quest_pack.v1` | engine `75` | `release.certify` | quest pack verdict | `deny.quest.certification_gap` |

## Exact compare modes
- quest-branch compare;
- checkpoint access compare;
- world consequence triplet compare;
- recovery compare;

## Required overlays and drilldowns
- quest graph;
- checkpoint access;
- world consequence deltas;
- persistent branch state;
- baseline anchors;

## Exact inspector fields
- quest profile id;
- consequence policy id;
- checkpoint id;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;

## Disabled reason families
- `disable.quest.profile_missing`;
- `disable.quest.policy_unbound`;
- `disable.quest.baseline_missing`;
- `disable.quest.route_blocked`;

## Focus and recovery law
- author/bind success -> stay in editor `79` with dirty profile and binding drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `79` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.quest.triplet` and pack `pack.quest_event_consequence` pinned;
- capture success -> `103` or `105` with capture mode `capture.quest.consequence_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `79` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.quest_event_consequence`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.quest.profile`;
- `artifact.quest.binding`;
- `artifact.quest.compare_digest`;
- `artifact.quest.trace_ref`;
- `artifact.quest.baseline_ptr`;

## Freeze relevance
- certification for `pack.quest_event_consequence` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.



## Phase-3 brutal proof slices
- delayed consequence chains remain restore-safe and replay-safe;
- world consequence may affect society or economy only through declared legality rows;
- every branch exports one reason backlink to `editor/80` and one evidence backlink to `editor/105`.

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
