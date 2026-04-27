# Generative Dialogue And Semantic Gate Catalog

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define the exact typed bridge for this heavy-domain family.
This file owns public packet families, field tables, code registries, compatibility law, replay/compare payload contracts, and evidence duties.

## Exact packet families
| Packet family | Mandatory payload meaning |
|---|---|
| `packet.semantic.chain_audit.v4` | intent scope, guard rows, consequence chain ids, baseline anchor |
| `packet.semantic.guard_failure.v3` | guard id, denial class, focus target, next action |
| `packet.semantic.compare_triplet.v3` | baseline/failed/recovery ids for semantic consequence review |

## Field-level schema table
| Field | Required |
|---|---|
| `intent_id` | yes |
| `speaker_id` | yes |
| `target_id` | yes |
| `policy_gate_id` | yes |
| `guard_tier` | yes |
| `consequence_chain_id` | yes |
| `memory_delta_id` | yes |
| `baseline_anchor_id` | yes |
| `replay_window_id` | yes |
| `first_failure_code` | yes |
| `next_action_id` | yes |
| `focus_target_id` | yes |

## Domain enum and failure-code registries
- guard_tier: permissive, standard, strict
- semantic_scope: utterance, exchange, consequence_chain
- consequence_class: local, persistent, cross_system

Failure codes:
- `semantic.policy.guard_fail`
- `semantic.consequence.drift`
- `semantic.baseline.missing`

## Replay payload contract
replay-bearing payloads must include turn sequence ids and baseline anchor identity.

## Compare payload contract
compare payloads must include intent digest, guard verdict digest, and consequence drift digest.

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
