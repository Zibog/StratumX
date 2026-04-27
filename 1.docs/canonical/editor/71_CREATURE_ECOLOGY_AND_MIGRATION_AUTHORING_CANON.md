# Creature Ecology And Migration Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for species profiles, migration corridors, hazard avoidance, predation pressure, and weather-aware rerouting. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.eco.author_species_profile` | `route.eco.author_species_profile.v1` | `packet.eco.author_species_profile.v1` | engine `66` | `mutate.profile` | species-profile revision | `deny.eco.species_profile_invalid` |
| bind | `btn.eco.bind_migration_corridor` | `route.eco.bind_migration_corridor.v1` | `packet.eco.bind_migration_corridor.v1` | engine `66` | `mutate.binding` | migration-corridor binding | `deny.eco.corridor_missing` |
| inspect | `btn.eco.inspect_hazard_chain` | `route.eco.inspect_hazard_chain.v1` | `packet.eco.inspect_hazard_chain.v1` | engine `66` | `read.inspect` | hazard-chain drilldown | `deny.eco.hazard_chain_missing` |
| simulate | `btn.eco.simulate_migration_window` | `route.eco.simulate_migration_window.v1` | `packet.eco.simulate_migration_window.v1` | engine `66` | `simulate.preview` | migration-window preview | `deny.eco.preview_scope_invalid` |
| compare | `btn.eco.compare_triplet` | `route.eco.compare_triplet.v1` | `packet.eco.compare_triplet.v1` | engine `66` | `analyze.compare` | ecology compare digest | `deny.eco.compare_baseline_missing` |
| capture | `btn.eco.capture_evidence` | `route.eco.capture_evidence.v1` | `packet.eco.capture_evidence.v1` | engine `66` | `capture.artifact` | ecology evidence bundle | `deny.eco.capture_target_missing` |
| recover | `btn.eco.recover_baseline` | `route.eco.recover_baseline.v1` | `packet.eco.recover_baseline.v1` | engine `66` | `recover.baseline` | recovered ecology baseline | `deny.eco.recovery_anchor_missing` |
| certify | `btn.eco.certify_pack` | `route.eco.certify_pack.v1` | `packet.eco.certify_pack.v1` | engine `66` | `release.certify` | ecology pack verdict | `deny.eco.certification_gap` |

## Exact compare modes
- corridor triplet compare;
- hazard reroute compare;
- predation pressure compare;
- recovery compare;

## Required overlays and drilldowns
- species profile;
- corridor graph;
- hazard chain;
- predation pressure field;
- recovery anchors;

## Exact inspector fields
- species profile id;
- migration corridor id;
- route graph state;
- hazard chain id;
- predation pressure state;
- baseline id;
- failed-run id;
- first blocker code;

## Disabled reason families
- `disable.eco.species_profile_missing`;
- `disable.eco.corridor_unbound`;
- `disable.eco.baseline_missing`;
- `disable.eco.route_blocked`;

## Phase-3 brutal proof slices
- hunger and weather reroute migration without editing hidden paths;
- predation and hazard avoidance coexist with corridor legality;
- every reroute exports one reason fragment to `engine/76`.

## Focus and recovery law
- author/bind success -> stay in editor `71` with dirty profile and corridor drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `71` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.eco.route_triplet` and pack `pack.ecology_migration` pinned;
- capture success -> `103` or `105` with capture mode `capture.eco.route_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `71` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.ecology_migration`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.eco.profile`;
- `artifact.eco.binding`;
- `artifact.eco.compare_digest`;
- `artifact.eco.route_trace_ref`;
- `artifact.eco.baseline_ptr`;

## Freeze relevance
- certification for `pack.ecology_migration` is freeze-relevant if the lab produces retained artifacts that participate in persistence or old-floor mixed packs;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
