# Quest Event And Systemic World Consequence Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define generic event, consequence and delayed-world-change truth for systemic gameplay without requiring scripted one-off paths.

## Exact truth objects
| Truth object | Role | Authoritative fields | Publication scope |
|---|---|---|---|
| `event_rule_profile` | Canonical event rule | `trigger_class`, `domain_bindings`, `guard_set`, `delay_window` | quest + world + diagnostics |
| `consequence_chain_state` | Applied and pending consequences | `chain_id`, `domain_targets`, `pending_window`, `rollback_anchor` | quest + persistence + reason |
| `delayed_consequence_anchor` | Deferred effect anchor | `release_condition`, `evidence_ref`, `baseline_ref`, `expiry_policy` | world consequence + recovery |
| `cross_domain_legality_row` | Cross-domain permission truth | `source_domain`, `target_domain`, `mutation_class`, `denial_family` | gameplay + audit |

## Exact phase order
| Phase | Input | Output | Illegal shortcut |
|---|---|---|---|
| `author_event_rule` | authored event intent | canonical event rule | script-only event path |
| `bind_cross_domain_legality` | domain link request | legality row | quest consequence without legality row |
| `simulate_consequence_chain` | event + legality rows | consequence chain state | silent delayed effect |
| `publish_consequence_trace` | chain result | reason/evidence/compare payload | consequence without why-chain |

## Coupling boundaries
| Boundary | Allowed through | Forbidden | Reason |
|---|---|---|---|
| quest -> society/economy | `packet.quest.bind_event_rule.v1` | hidden economy or crime shift | cross-domain legality must be explicit |
| quest -> persistence | `event.quest.persist_required.v1` | non-retained delayed consequence | afterlife must remain coherent |
| quest -> causality | `event.quest.reason_published.v1` | invisible explanation path | operator must explain chain |

## Failure and denial families
| Family | Meaning | Retryable | Required artifact / trace |
|---|---|---|---|
| `fail.event_rule_missing` | event has no canonical rule | no | `artifact.quest.rule`, `trace.quest.lookup` |
| `fail.cross_domain_legality_denied` | rule exists but cross-domain mutation is illegal | no | `artifact.quest.legality`, `trace.quest.denial` |
| `fail.delayed_anchor_invalid` | deferred chain missing legal anchor | yes | `artifact.quest.delay`, `trace.quest.delay` |

## Resource envelope and degrade law
| Axis | Nominal law | Degrade rung | May never be faked |
|---|---|---|---|
| CPU | chain solve bounded by affected domains and delay windows | drop non-critical preview branches | legality and pending consequence set |
| GPU | presentation only | drop cosmetic outcome widgets | consequence truth |
| RAM | deferred anchors tiered hot/warm/cold | demote expired preview context first | active pending anchors |
| Disk/IO | chain persistence journaled | delay non-cert transcript export | baseline/recovery triplets |

## Publication and evidence obligations
| Event / artifact | Publisher | Required payload | Consumer |
|---|---|---|---|
| `event.quest.chain_applied.v1` | engine/75 | `chain_id`, `target_domains`, `delay_window`, `trace_ref`, `artifact_ref` | sdk/76 + editor/79 |
| `artifact.quest.chain.triplet` | tooling | baseline/failed/recovery, compare digest, evidence digest | editor/105 + editor/109 |

## Legal recovery actions
| Failure family | Legal recovery | Required anchor | Next legal focus |
|---|---|---|---|
| `fail.event_rule_missing` | author event rule | quest rule baseline | `editor/79` |
| `fail.cross_domain_legality_denied` | inspect legality, rebind allowed domains, rerun | legality artifact | `editor/79_then_80` |
| `fail.delayed_anchor_invalid` | recover delayed anchor then compare | failed-run id + baseline bundle | `editor/79_then_105` |



## Exact editor entrypoints
- `btn.quest.author_quest_profile`
- `btn.quest.bind_consequence_policy`
- `btn.quest.inspect_checkpoint_chain`
- `btn.quest.simulate_event_branch`
- `btn.quest.compare_consequence_triplet`
- `btn.quest.capture_consequence_evidence`
- `btn.quest.recover_consequence_baseline`
- `btn.quest.certify_quest_pack`

## Phase-3 proof slices
- delayed consequence uses one restore-safe anchor and one explicit legality row;
- cross-domain consequence may not mutate economy, society, or inventory silently;
- every quest branch exports one reason fragment to `engine/76`.

## Required publications
- `event.quest.chain_applied.v1` with `chain_id`, `delay_window`, `rollback_anchor`, `trace_ref`, `artifact_ref`;
- `artifact.quest.chain.triplet` with baseline/failed/recovery and compare digest.


## Status
`document_gold / doc_closed_impl_open`
