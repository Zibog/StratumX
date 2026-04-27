# Animation Synthesis Micro Motion And Contact Intent Canon

**Stack version:** `SX-CANON/1.0.24/STACK-v30`

## Purpose
Define generic animation-intent truth, micro-motion variance, contact targets and interaction legality without binding motion grammar to a single product scene.

## Exact truth objects
| Truth object | Role | Authoritative fields | Publication scope |
|---|---|---|---|
| `interaction_target_grammar` | Legal reach/grab/open targets | `target_class`, `handle_type`, `contact_zone`, `fallback_rule` | animation + interaction + editor |
| `motion_variance_profile` | Per skeleton / species micro-motion | `skeleton_family`, `variance_band`, `reach_tolerance`, `pose_bias` | runtime solve + compare |
| `contact_intent_state` | Current intended contact | `intent_class`, `target_ref`, `approach_window`, `commit_window` | runtime + diagnostics |
| `fallback_motion_state` | Legal fallback chain | `fallback_rung`, `degrade_reason`, `rebind_target`, `recovery_anchor` | runtime + recovery |

## Exact phase order
| Phase | Input | Output | Illegal shortcut |
|---|---|---|---|
| `load_target_grammar` | authored interaction data | target grammar row | implicit target inference without row |
| `bind_variance_profile` | skeleton/species row | active motion variance | species-specific variance bypass |
| `solve_reach_and_contact` | intent state + active profile | pose/contact solve | reach target without legality check |
| `publish_interaction_trace` | solve result | trace + compare payload + artifact ref | contact resolution without trace anchor |

## Coupling boundaries
| Boundary | Allowed through | Forbidden | Reason |
|---|---|---|---|
| animation -> interaction | `packet.anim.bind_target_grammar.v1` | direct runtime contact override | interaction legality lives in truth rows |
| animation -> skinning | `event.anim.pose_committed.v1` | presentation-only offset as truth | pose commit is downstream of truth solve |
| animation -> causality | `event.interaction.intent_resolved.v1` | silent interaction consequence | operator needs explainability |

## Failure and denial families
| Family | Meaning | Retryable | Required artifact / trace |
|---|---|---|---|
| `fail.target_grammar_missing` | interaction target lacks canonical grammar | no | `artifact.anim.grammar`, `trace.anim.target_lookup` |
| `fail.reach_out_of_legal_window` | target exists but current solve exceeds tolerance | yes | `artifact.anim.solve`, `trace.anim.reach_window` |
| `fail.fallback_chain_exhausted` | legal fallback ladder ran out | yes | `artifact.anim.recovery`, `trace.anim.fallback` |

## Resource envelope and degrade law
| Axis | Nominal law | Degrade rung | May never be faked |
|---|---|---|---|
| CPU | contact solve bounded by target subset | drop higher-order micro variance first | contact legality and recovery rung |
| GPU | presentation may simplify skinning extras | drop cosmetic secondary motion | contact solve result |
| RAM | retain baseline/failed/recovery motion bundles | demote old preview bundles | last-good interaction baseline |
| Disk/IO | capture bundles are compact triplets | delay non-cert preview export | certification evidence bundle |

## Publication and evidence obligations
| Event / artifact | Publisher | Required payload | Consumer |
|---|---|---|---|
| `event.anim.contact_resolved.v1` | engine/70 | `intent_class`, `target_ref`, `solve_rung`, `artifact_ref`, `trace_ref` | sdk/77 + editor/75 |
| `artifact.anim.interaction.triplet` | tooling | baseline/failed/recovery, compare digest, motion law revision | editor/105 + editor/109 |

## Legal recovery actions
| Failure family | Legal recovery | Required anchor | Next legal focus |
|---|---|---|---|
| `fail.target_grammar_missing` | author target grammar | interaction grammar baseline | `editor/75` |
| `fail.reach_out_of_legal_window` | adjust variance / target class then simulate | failed-run id | `editor/75_then_100` |
| `fail.fallback_chain_exhausted` | recover baseline and rebind fallback rung | recovery bundle | `editor/75_then_105` |

## Status
`document_gold / doc_closed_impl_open`
