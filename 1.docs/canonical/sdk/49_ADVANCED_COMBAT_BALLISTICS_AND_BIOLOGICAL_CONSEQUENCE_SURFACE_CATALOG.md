# Advanced Combat Ballistics And Biological Consequence Surface Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact typed bridge for this heavy-domain family.
This file owns public packet families, field tables, code registries, compatibility law, replay/compare payload contracts, and evidence duties.

## Exact packet families
| Packet family | Mandatory payload meaning |
|---|---|
| `packet.wound.trace_review.v4` | trace-link rows, wound region digest, species profile ref |
| `packet.ballistics.replay_triplet.v3` | baseline/failed/recovery ids for ballistic+wound replay |
| `packet.creature.damage_route.v3` | creature damage scope, lethality class, next action |

## Field-level schema table
| Field | Required |
|---|---|
| `trace_id` | yes |
| `actor_id` | yes |
| `species_profile_id` | yes |
| `anatomical_region_id` | yes |
| `armor_class` | yes |
| `penetration_band` | yes |
| `bleed_shock_tier` | yes |
| `dismember_rule_id` | yes |
| `baseline_id` | yes |
| `failed_run_id` | yes |
| `recovery_run_id` | yes |
| `first_failure_code` | yes |

## Domain enum and failure-code registries
- penetration_band: glance, partial, full
- bleed_shock_tier: none, low, medium, critical
- lethality_class: survive, unstable, lethal

Failure codes:
- `wound.region.link_missing`
- `wound.replay_gap`
- `wound.dismember.denied`

## Replay payload contract
replay-bearing payloads must keep stable trace and region ids across retries and comparison.

## Compare payload contract
compare payloads must expose both wound-region and lethality-profile digests.

## Compatibility and version rules
- any field-order, semantics, enum expansion, or lifecycle change that affects compare or replay meaning requires a version bump;
- normalization may compact payload shape only if the raw values, first failure code, next action id, and focus target id remain present;
- consumers may not infer unnamed payloads, unnamed artifacts, or unnamed codes.

## Evidence duties
- every certification-bearing or compare-bearing packet must preserve retained artifact references;
- every terminal failure must preserve first failure code and next legal action;
- every terminal success that participates in certification must preserve compare ids and evidence posture;
- lifecycle semantics must agree with sdk `77`.

## Current posture
`document_gold / doc_closed_impl_open`
