# Inventory Loot Equipment And Economy Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define generic inventory, container persistence, trader economy and scarcity-loop truth across systemic world simulation.

## Exact truth objects
| Truth object | Role | Authoritative fields | Publication scope |
|---|---|---|---|
| `container_inventory_state` | Persistent container truth | `capacity_class`, `contents_digest`, `persistence_tier`, `owner_scope` | inventory + persistence |
| `equipment_profile` | Equippable legality | `slot_class`, `stack_rules`, `armor_interaction`, `degrade_policy` | inventory + combat |
| `scarcity_loop_state` | Economy and scarcity truth | `commodity_band`, `price_response`, `crime_pressure_link`, `faction_response` | economy + society |
| `trade_reaction_anchor` | World event reaction row | `event_class`, `inventory_delta`, `price_delta`, `baseline_ref` | economy + recovery |

## Exact phase order
| Phase | Input | Output | Illegal shortcut |
|---|---|---|---|
| `author_equipment_profile` | equipment law request | canonical profile | equip legality outside profile registry |
| `bind_container_state` | container + persistence request | persistent inventory row | container truth bypassing persistence |
| `simulate_trade_or_scarcity` | economy event / world delta | trade or scarcity delta | price shift without reaction anchor |
| `publish_inventory_consequence` | inventory/economy delta | diagnostics + compare/evidence bundle | loot/economy change without trace |

## Coupling boundaries
| Boundary | Allowed through | Forbidden | Reason |
|---|---|---|---|
| inventory -> society | `packet.inv.author_scarcity_loop.v1` | crime pressure hardcoded in quest layer | scarcity affects behavior truth |
| inventory -> wound/armor | `event.equipment.profile_published.v1` | armor effect outside equipment truth | ballistics requires canonical equipment profile |
| inventory -> persistence | `event.inventory.persist_required.v1` | container mutation lost on restore | inventory must survive restore |

## Failure and denial families
| Family | Meaning | Retryable | Required artifact / trace |
|---|---|---|---|
| `fail.container_state_missing` | container lacks canonical state | no | `artifact.inv.container`, `trace.inv.lookup` |
| `fail.equipment_profile_invalid` | slot/armor/degrade rule mismatch | yes | `artifact.inv.profile`, `trace.inv.validation` |
| `fail.trade_reaction_anchor_missing` | world event cannot propagate legally | yes | `artifact.inv.reaction`, `trace.inv.trade` |

## Resource envelope and degrade law
| Axis | Nominal law | Degrade rung | May never be faked |
|---|---|---|---|
| CPU | economy sim bounded by affected commodity and zone sets | drop non-critical trader preview loops | container persistence and legality |
| GPU | presentation only | drop non-critical store overlays | inventory truth |
| RAM | persistent container state tiered by importance | demote cold market histories first | last-good container baselines |
| Disk/IO | inventory deltas journaled chunk-wise | defer low-value ledger export | trade reaction anchors |

## Publication and evidence obligations
| Event / artifact | Publisher | Required payload | Consumer |
|---|---|---|---|
| `event.inv.container_or_trade_delta.v1` | engine/74 | `container_ref`, `commodity_band`, `trace_ref`, `artifact_ref`, `failed_run_id?` | sdk/73 + editor/78 |
| `artifact.inv.trade_triplet` | tooling | baseline/failed/recovery, scarcity digest, evidence refs | editor/105 + editor/109 |

## Legal recovery actions
| Failure family | Legal recovery | Required anchor | Next legal focus |
|---|---|---|---|
| `fail.container_state_missing` | bind container state | inventory baseline | `editor/78` |
| `fail.equipment_profile_invalid` | author valid equipment profile then compare | profile artifact | `editor/78_then_105` |
| `fail.trade_reaction_anchor_missing` | recover baseline and rerun world-event trade reaction | failed-run id | `editor/78_then_100` |



## Exact editor entrypoints
- `btn.inv.author_item_profile`
- `btn.inv.bind_trader_policy`
- `btn.inv.inspect_loadout_legality`
- `btn.inv.simulate_trade_flow`
- `btn.inv.compare_inventory_triplet`
- `btn.inv.capture_inventory_evidence`
- `btn.inv.recover_inventory_baseline`
- `btn.inv.certify_inventory_pack`

## Phase-3 proof slices
- scarcity loops alter trader response and social pressure lawfully;
- equipment truth remains the only legal path for armor interaction;
- every inventory/economy reaction exports one reason fragment to `engine/76`.

## Required publications
- `event.inv.inventory_or_trade_delta.v1` with `container_ref`, `equipment_ref`, `commodity_band`, `trace_ref`, `artifact_ref`;
- `artifact.inv.trade.triplet` with baseline/failed/recovery and scarcity digest.


## Status
`document_gold / doc_closed_impl_open`
