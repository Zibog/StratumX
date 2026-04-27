# Inventory Loot Trader And Equipment Production Lab Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Own the full production contour for inventory, equipment, trader economy, and loot authoring. This lab is not review-only: it must let an operator author, bind, inspect, simulate, compare, capture, recover, and certify without improvising outside canonical routes.

## Full production command contour
| Class | button id | tooling route | sdk family | engine truth owner | mutation class | primary outcome | first denial family |
|---|---|---|---|---|---|---|---|
| author | `btn.inv.author_item_profile` | `route.inv.author_item_profile.v1` | `packet.inv.author_item_profile.v1` | engine `74` | `mutate.profile` | item profile revision | `deny.inv.item_profile_invalid` |
| bind | `btn.inv.bind_trader_policy` | `route.inv.bind_trader_policy.v1` | `packet.inv.bind_trader_policy.v1` | engine `74` | `mutate.binding` | trader policy binding | `deny.inv.trader_policy_missing` |
| inspect | `btn.inv.inspect_loadout_legality` | `route.inv.inspect_loadout_legality.v1` | `packet.inv.inspect_loadout_legality.v1` | engine `74` | `read.inspect` | loadout legality drilldown | `deny.inv.loadout_missing` |
| simulate | `btn.inv.simulate_trade_flow` | `route.inv.simulate_trade_flow.v1` | `packet.inv.simulate_trade_flow.v1` | engine `74` | `simulate.preview` | trade-flow preview | `deny.inv.trade_scope_invalid` |
| compare | `btn.inv.compare_inventory_triplet` | `route.inv.compare_inventory_triplet.v1` | `packet.inv.compare_inventory_triplet.v1` | engine `74` | `analyze.compare` | inventory compare digest | `deny.inv.compare_baseline_missing` |
| capture | `btn.inv.capture_inventory_evidence` | `route.inv.capture_inventory_evidence.v1` | `packet.inv.capture_inventory_evidence.v1` | engine `74` | `capture.artifact` | inventory evidence bundle | `deny.inv.capture_target_missing` |
| recover | `btn.inv.recover_inventory_baseline` | `route.inv.recover_inventory_baseline.v1` | `packet.inv.recover_inventory_baseline.v1` | engine `74` | `recover.baseline` | recovered inventory baseline | `deny.inv.recovery_anchor_missing` |
| certify | `btn.inv.certify_inventory_pack` | `route.inv.certify_inventory_pack.v1` | `packet.inv.certify_inventory_pack.v1` | engine `74` | `release.certify` | inventory pack verdict | `deny.inv.certification_gap` |

## Exact compare modes
- item identity compare;
- loadout legality compare;
- trade-flow triplet compare;
- checkpoint recovery compare;

## Required overlays and drilldowns
- item identities;
- loadout legality;
- economy deltas;
- checkpoint lineage;
- baseline anchors;

## Exact inspector fields
- item profile id;
- trader policy id;
- loadout id;
- baseline id;
- failed-run id;
- recovery-run id;
- first blocker code;

## Disabled reason families
- `disable.inv.profile_missing`;
- `disable.inv.trader_unbound`;
- `disable.inv.baseline_missing`;
- `disable.inv.trade_route_blocked`;

## Focus and recovery law
- author/bind success -> stay in editor `78` with dirty profile and binding drilldown visible;
- inspect/simulate success -> route focus to compare or capture inside editor `78` or `103` depending on artifact posture;
- compare success -> `105` with compare mode `compare.inv.triplet` and pack `pack.inventory_equipment_economy` pinned;
- capture success -> `103` or `105` with capture mode `capture.inv.evidence_bundle` and retained artifact refs visible;
- recover success -> rerun compare in editor `78` using the same baseline family;
- certify success -> `103`, then `109` if freeze relevance is active for `pack.inventory_equipment_economy`;
- retryable failure -> remain in the owning lab with one explicit next legal recovery action;
- terminal failure -> reveal first failure code, blocker trace, required artifact refs, and last-good baseline pointer before signoff stays available;

## Retained artifacts
- `artifact.inv.profile`;
- `artifact.inv.binding`;
- `artifact.inv.compare_digest`;
- `artifact.inv.trace_ref`;
- `artifact.inv.baseline_ptr`;

## Freeze relevance
- certification for `pack.inventory_equipment_economy` is freeze-relevant if the lab produces retained artifacts that participate in mixed floor packs or release signoff bundles;
- no freeze review may hide first failure code or degrade ladder rung;
- disabled reasons, compare modes, and recovery actions must match the exact button manifest in `editor/110`.



## Phase-3 brutal proof slices
- persistent container state survives compare/recover without hidden mutation;
- scarcity loop changes trader economy lawfully and can be explained;
- equipment legality remains the only canonical route for armor interaction.

## Current posture
`document_gold / doc_closed_impl_open / production_surface_closed_in_docs`
